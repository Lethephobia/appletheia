mod transfer_error;
mod transfer_event_payload;
mod transfer_event_payload_error;
mod transfer_id;
mod transfer_state;
mod transfer_state_error;
mod transfer_status;

pub use transfer_error::TransferError;
pub use transfer_event_payload::TransferEventPayload;
pub use transfer_event_payload_error::TransferEventPayloadError;
pub use transfer_id::TransferId;
pub use transfer_state::TransferState;
pub use transfer_state_error::TransferStateError;
pub use transfer_status::TransferStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::{AccountBalance, AccountId};

/// Represents the `Transfer` aggregate root.
#[aggregate(type = "transfer", error = TransferError)]
pub struct Transfer {
    core: AggregateCore<TransferState, TransferEventPayload>,
}

impl Transfer {
    /// Initiates a new transfer.
    pub fn initiate(
        &mut self,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: AccountBalance,
    ) -> Result<(), TransferError> {
        if self.state().is_some() {
            return Err(TransferError::AlreadyInitiated);
        }

        if from_account_id == to_account_id {
            return Err(TransferError::SameAccount);
        }

        if amount.is_zero() {
            return Err(TransferError::ZeroAmount);
        }

        self.append_event(TransferEventPayload::Initiated {
            id: TransferId::new(),
            from_account_id,
            to_account_id,
            amount,
        })
    }

    /// Marks the transfer as completed.
    pub fn mark_completed(&mut self) -> Result<(), TransferError> {
        match self.state_required()?.status() {
            TransferStatus::Pending => self.append_event(TransferEventPayload::Completed),
            TransferStatus::Completed => Ok(()),
            TransferStatus::Failed => Err(TransferError::AlreadyFailed),
            TransferStatus::Cancelled => Err(TransferError::AlreadyCancelled),
        }
    }

    /// Marks the transfer as failed.
    pub fn mark_failed(&mut self) -> Result<(), TransferError> {
        match self.state_required()?.status() {
            TransferStatus::Pending => self.append_event(TransferEventPayload::Failed),
            TransferStatus::Completed => Err(TransferError::AlreadyCompleted),
            TransferStatus::Failed => Ok(()),
            TransferStatus::Cancelled => Err(TransferError::AlreadyCancelled),
        }
    }

    /// Cancels the transfer.
    pub fn cancel(&mut self) -> Result<(), TransferError> {
        match self.state_required()?.status() {
            TransferStatus::Pending => self.append_event(TransferEventPayload::Cancelled),
            TransferStatus::Completed => Err(TransferError::AlreadyCompleted),
            TransferStatus::Failed => Err(TransferError::AlreadyFailed),
            TransferStatus::Cancelled => Ok(()),
        }
    }
}

impl AggregateApply<TransferEventPayload, TransferError> for Transfer {
    fn apply(&mut self, payload: &TransferEventPayload) -> Result<(), TransferError> {
        match payload {
            TransferEventPayload::Initiated {
                id,
                from_account_id,
                to_account_id,
                amount,
            } => {
                self.set_state(Some(TransferState::new(
                    *id,
                    *from_account_id,
                    *to_account_id,
                    *amount,
                )));
            }
            TransferEventPayload::Completed => {
                self.state_required_mut()?.mark_completed();
            }
            TransferEventPayload::Failed => {
                self.state_required_mut()?.mark_failed();
            }
            TransferEventPayload::Cancelled => {
                self.state_required_mut()?.cancel();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateState, Event, EventPayload};

    use crate::account::{AccountBalance, AccountId};

    use super::{Transfer, TransferEventPayload, TransferId, TransferStatus};

    #[test]
    fn initiate_initializes_state_and_records_event() {
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let amount = AccountBalance::new(100);
        let mut transfer = Transfer::default();

        transfer
            .initiate(from_account_id, to_account_id, amount)
            .expect("initiate should succeed");

        let state = transfer.state().expect("state should exist");
        assert_eq!(
            state.id(),
            transfer.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(state.from_account_id(), &from_account_id);
        assert_eq!(state.to_account_id(), &to_account_id);
        assert_eq!(state.amount(), &amount);
        assert_eq!(state.status(), &TransferStatus::Pending);
        assert_eq!(transfer.uncommitted_events().len(), 1);
        assert_eq!(
            transfer.uncommitted_events()[0].payload().name(),
            TransferEventPayload::INITIATED
        );
    }

    #[test]
    fn initiate_rejects_same_account_transfer() {
        let account_id = AccountId::new();
        let mut transfer = Transfer::default();

        let error = transfer
            .initiate(account_id, account_id, AccountBalance::new(1))
            .expect_err("same-account transfer should fail");

        assert!(matches!(error, super::TransferError::SameAccount));
    }

    #[test]
    fn mark_completed_updates_status() {
        let mut transfer = Transfer::default();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        transfer
            .initiate(from_account_id, to_account_id, AccountBalance::new(100))
            .expect("initiate should succeed");

        transfer
            .mark_completed()
            .expect("mark completed should succeed");

        assert_eq!(
            transfer.state().expect("state should exist").status(),
            &TransferStatus::Completed
        );
    }

    #[test]
    fn mark_failed_updates_status() {
        let mut transfer = Transfer::default();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        transfer
            .initiate(from_account_id, to_account_id, AccountBalance::new(100))
            .expect("initiate should succeed");

        transfer.mark_failed().expect("mark failed should succeed");

        assert_eq!(
            transfer.state().expect("state should exist").status(),
            &TransferStatus::Failed
        );
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let id = TransferId::new();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let initiated = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            TransferEventPayload::Initiated {
                id,
                from_account_id,
                to_account_id,
                amount: AccountBalance::new(100),
            },
        );
        let completed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            TransferEventPayload::Completed,
        );
        let mut transfer = Transfer::default();

        transfer
            .replay_events(vec![initiated, completed], None)
            .expect("events should replay");

        let state = transfer.state().expect("state should exist");
        assert_eq!(state.from_account_id(), &from_account_id);
        assert_eq!(state.to_account_id(), &to_account_id);
        assert_eq!(state.amount(), &AccountBalance::new(100));
        assert_eq!(state.status(), &TransferStatus::Completed);
        assert_eq!(transfer.version().value(), 2);
        assert!(transfer.uncommitted_events().is_empty());
    }
}
