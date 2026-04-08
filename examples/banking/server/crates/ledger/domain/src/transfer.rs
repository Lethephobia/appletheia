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

use crate::account::AccountId;
use crate::core::CurrencyAmount;

/// Represents the `Transfer` aggregate root.
#[aggregate(type = "transfer", error = TransferError)]
pub struct Transfer {
    core: AggregateCore<TransferState, TransferEventPayload>,
}

impl Transfer {
    /// Returns the source account.
    pub fn from_account_id(&self) -> Result<&AccountId, TransferError> {
        Ok(&self.state_required()?.from_account_id)
    }

    /// Returns the destination account.
    pub fn to_account_id(&self) -> Result<&AccountId, TransferError> {
        Ok(&self.state_required()?.to_account_id)
    }

    /// Returns the transfer amount.
    pub fn amount(&self) -> Result<&CurrencyAmount, TransferError> {
        Ok(&self.state_required()?.amount)
    }

    /// Returns the current transfer status.
    pub fn status(&self) -> Result<&TransferStatus, TransferError> {
        Ok(&self.state_required()?.status)
    }

    /// Requests a new transfer.
    pub fn request(
        &mut self,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<(), TransferError> {
        if self.state().is_some() {
            return Err(TransferError::AlreadyRequested);
        }

        if from_account_id == to_account_id {
            return Err(TransferError::SameAccount);
        }

        if amount.is_zero() {
            return Err(TransferError::ZeroAmount);
        }

        self.append_event(TransferEventPayload::Requested {
            id: TransferId::new(),
            from_account_id,
            to_account_id,
            amount,
        })
    }

    /// Completes the transfer.
    pub fn complete(&mut self) -> Result<(), TransferError> {
        self.ensure_pending()?;

        self.append_event(TransferEventPayload::Completed)
    }

    /// Fails the transfer.
    pub fn fail(&mut self) -> Result<(), TransferError> {
        self.ensure_pending()?;

        self.append_event(TransferEventPayload::Failed)
    }

    /// Cancels the transfer.
    pub fn cancel(&mut self) -> Result<(), TransferError> {
        self.ensure_pending()?;

        self.append_event(TransferEventPayload::Cancelled)
    }

    fn ensure_pending(&self) -> Result<(), TransferError> {
        match self.state_required()?.status {
            TransferStatus::Pending => Ok(()),
            TransferStatus::Completed => Err(TransferError::AlreadyCompleted),
            TransferStatus::Failed => Err(TransferError::AlreadyFailed),
            TransferStatus::Cancelled => Err(TransferError::AlreadyCancelled),
        }
    }
}

impl AggregateApply<TransferEventPayload, TransferError> for Transfer {
    fn apply(&mut self, payload: &TransferEventPayload) -> Result<(), TransferError> {
        match payload {
            TransferEventPayload::Requested {
                id,
                from_account_id,
                to_account_id,
                amount,
            } => self.set_state(Some(TransferState::new(
                *id,
                *from_account_id,
                *to_account_id,
                *amount,
            ))),
            TransferEventPayload::Completed => {
                self.state_required_mut()?.status = TransferStatus::Completed;
            }
            TransferEventPayload::Failed => {
                self.state_required_mut()?.status = TransferStatus::Failed;
            }
            TransferEventPayload::Cancelled => {
                self.state_required_mut()?.status = TransferStatus::Cancelled;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;

    use super::{Transfer, TransferEventPayload, TransferId, TransferStatus};

    #[test]
    fn request_initializes_state_and_records_event() {
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let amount = CurrencyAmount::new(100);
        let mut transfer = Transfer::default();

        transfer
            .request(from_account_id, to_account_id, amount)
            .expect("request should succeed");

        assert_eq!(
            transfer.aggregate_id().expect("aggregate id should exist"),
            transfer.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(
            transfer
                .from_account_id()
                .expect("from account id should exist"),
            &from_account_id
        );
        assert_eq!(
            transfer
                .to_account_id()
                .expect("to account id should exist"),
            &to_account_id
        );
        assert_eq!(transfer.amount().expect("amount should exist"), &amount);
        assert_eq!(
            transfer.status().expect("status should exist"),
            &TransferStatus::Pending
        );
        assert_eq!(transfer.uncommitted_events().len(), 1);
        assert_eq!(
            transfer.uncommitted_events()[0].payload().name(),
            TransferEventPayload::REQUESTED
        );
    }

    #[test]
    fn request_rejects_same_account_transfer() {
        let account_id = AccountId::new();
        let mut transfer = Transfer::default();

        let error = transfer
            .request(account_id, account_id, CurrencyAmount::new(1))
            .expect_err("same-account transfer should fail");

        assert!(matches!(error, super::TransferError::SameAccount));
    }

    #[test]
    fn complete_updates_status() {
        let mut transfer = Transfer::default();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        transfer
            .request(from_account_id, to_account_id, CurrencyAmount::new(100))
            .expect("request should succeed");

        transfer.complete().expect("complete should succeed");
        let duplicate_complete_error = transfer
            .complete()
            .expect_err("duplicate complete should fail");

        assert_eq!(
            transfer.status().expect("status should exist"),
            &TransferStatus::Completed
        );
        assert!(matches!(
            duplicate_complete_error,
            super::TransferError::AlreadyCompleted
        ));
    }

    #[test]
    fn fail_updates_status() {
        let mut transfer = Transfer::default();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        transfer
            .request(from_account_id, to_account_id, CurrencyAmount::new(100))
            .expect("request should succeed");

        transfer.fail().expect("fail should succeed");
        let duplicate_fail_error = transfer.fail().expect_err("duplicate fail should fail");

        assert_eq!(
            transfer.status().expect("status should exist"),
            &TransferStatus::Failed
        );
        assert!(matches!(
            duplicate_fail_error,
            super::TransferError::AlreadyFailed
        ));
    }

    #[test]
    fn cancel_updates_status() {
        let mut transfer = Transfer::default();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        transfer
            .request(from_account_id, to_account_id, CurrencyAmount::new(100))
            .expect("request should succeed");

        transfer.cancel().expect("cancel should succeed");
        let duplicate_cancel_error = transfer.cancel().expect_err("duplicate cancel should fail");

        assert_eq!(
            transfer.status().expect("status should exist"),
            &TransferStatus::Cancelled
        );
        assert!(matches!(
            duplicate_cancel_error,
            super::TransferError::AlreadyCancelled
        ));
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let id = TransferId::new();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let requested = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            TransferEventPayload::Requested {
                id,
                from_account_id,
                to_account_id,
                amount: CurrencyAmount::new(100),
            },
        );
        let completed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            TransferEventPayload::Completed,
        );
        let mut transfer = Transfer::default();

        transfer
            .replay_events(vec![requested, completed], None)
            .expect("events should replay");

        assert_eq!(
            transfer
                .from_account_id()
                .expect("from account id should exist"),
            &from_account_id
        );
        assert_eq!(
            transfer
                .to_account_id()
                .expect("to account id should exist"),
            &to_account_id
        );
        assert_eq!(
            transfer.amount().expect("amount should exist"),
            &CurrencyAmount::new(100)
        );
        assert_eq!(
            transfer.status().expect("status should exist"),
            &TransferStatus::Completed
        );
        assert_eq!(transfer.version().value(), 2);
        assert!(transfer.uncommitted_events().is_empty());
    }
}
