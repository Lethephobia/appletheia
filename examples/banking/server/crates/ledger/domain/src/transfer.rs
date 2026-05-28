mod transfer_complete_rejection_reason;
mod transfer_complete_result;
mod transfer_error;
mod transfer_event_payload;
mod transfer_event_payload_error;
mod transfer_fail_rejection_reason;
mod transfer_fail_result;
mod transfer_failure_reason;
mod transfer_id;
mod transfer_request;
mod transfer_request_rejection_reason;
mod transfer_request_result;
mod transfer_state;
mod transfer_state_error;
mod transfer_status;

pub use transfer_complete_rejection_reason::TransferCompleteRejectionReason;
pub use transfer_complete_result::TransferCompleteResult;
pub use transfer_error::TransferError;
pub use transfer_event_payload::TransferEventPayload;
pub use transfer_event_payload_error::TransferEventPayloadError;
pub use transfer_fail_rejection_reason::TransferFailRejectionReason;
pub use transfer_fail_result::TransferFailResult;
pub use transfer_failure_reason::TransferFailureReason;
pub use transfer_id::TransferId;
pub use transfer_request::TransferRequest;
pub use transfer_request_rejection_reason::TransferRequestRejectionReason;
pub use transfer_request_result::TransferRequestResult;
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
        request: TransferRequest,
    ) -> Result<TransferRequestResult, TransferError> {
        if self.state().is_some() {
            return Err(TransferError::AlreadyRequested);
        }

        if request.is_same_account() {
            let reason = TransferRequestRejectionReason::SameAccount;
            let transfer_id = self.reject_request(request, reason)?;
            return Ok(TransferRequestResult::Rejected {
                transfer_id,
                reason,
            });
        }

        if request.amount().is_zero() {
            let reason = TransferRequestRejectionReason::ZeroAmount;
            let transfer_id = self.reject_request(request, reason)?;
            return Ok(TransferRequestResult::Rejected {
                transfer_id,
                reason,
            });
        }

        let transfer_id = TransferId::new();
        let (from_account_id, to_account_id, amount) = request.into_parts();
        self.append_event(TransferEventPayload::Requested {
            id: transfer_id,
            from_account_id,
            to_account_id,
            amount,
        })?;

        Ok(TransferRequestResult::Requested { transfer_id })
    }

    /// Rejects a transfer request.
    pub fn reject_request(
        &mut self,
        request: TransferRequest,
        reason: TransferRequestRejectionReason,
    ) -> Result<TransferId, TransferError> {
        let transfer_id = TransferId::new();
        let (from_account_id, to_account_id, amount) = request.into_parts();
        self.append_event(TransferEventPayload::RequestRejected {
            id: transfer_id,
            from_account_id,
            to_account_id,
            amount,
            reason,
        })?;
        Ok(transfer_id)
    }

    /// Completes the transfer.
    pub fn complete(&mut self) -> Result<TransferCompleteResult, TransferError> {
        match self.state_required()?.status {
            TransferStatus::Pending => {}
            TransferStatus::Completed => {
                let reason = TransferCompleteRejectionReason::AlreadyCompleted;
                self.reject_complete(reason)?;
                return Ok(TransferCompleteResult::Rejected { reason });
            }
            TransferStatus::Failed => {
                let reason = TransferCompleteRejectionReason::AlreadyFailed;
                self.reject_complete(reason)?;
                return Ok(TransferCompleteResult::Rejected { reason });
            }
            TransferStatus::Rejected => {
                let reason = TransferCompleteRejectionReason::AlreadyRejected;
                self.reject_complete(reason)?;
                return Ok(TransferCompleteResult::Rejected { reason });
            }
        }

        self.append_event(TransferEventPayload::Completed)?;

        Ok(TransferCompleteResult::Completed)
    }

    /// Rejects completing a transfer.
    pub fn reject_complete(
        &mut self,
        reason: TransferCompleteRejectionReason,
    ) -> Result<(), TransferError> {
        self.append_event(TransferEventPayload::CompleteRejected { reason })?;
        Ok(())
    }

    /// Fails the transfer.
    pub fn fail(
        &mut self,
        reason: TransferFailureReason,
    ) -> Result<TransferFailResult, TransferError> {
        match self.state_required()?.status {
            TransferStatus::Pending => {}
            TransferStatus::Completed => {
                let reason = TransferFailRejectionReason::AlreadyCompleted;
                self.reject_fail(reason)?;
                return Ok(TransferFailResult::Rejected { reason });
            }
            TransferStatus::Failed => {
                let reason = TransferFailRejectionReason::AlreadyFailed;
                self.reject_fail(reason)?;
                return Ok(TransferFailResult::Rejected { reason });
            }
            TransferStatus::Rejected => {
                let reason = TransferFailRejectionReason::AlreadyRejected;
                self.reject_fail(reason)?;
                return Ok(TransferFailResult::Rejected { reason });
            }
        }

        self.append_event(TransferEventPayload::Failed { reason })?;

        Ok(TransferFailResult::Failed)
    }

    /// Rejects failing a transfer.
    pub fn reject_fail(
        &mut self,
        reason: TransferFailRejectionReason,
    ) -> Result<(), TransferError> {
        self.append_event(TransferEventPayload::FailRejected { reason })?;
        Ok(())
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
            } => self.set_state(Some(TransferState {
                id: *id,
                from_account_id: *from_account_id,
                to_account_id: *to_account_id,
                amount: *amount,
                status: TransferStatus::Pending,
            })),
            TransferEventPayload::RequestRejected {
                id,
                from_account_id,
                to_account_id,
                amount,
                ..
            } => self.set_state(Some(TransferState {
                id: *id,
                from_account_id: *from_account_id,
                to_account_id: *to_account_id,
                amount: *amount,
                status: TransferStatus::Rejected,
            })),
            TransferEventPayload::Completed => {
                self.state_required_mut()?.status = TransferStatus::Completed;
            }
            TransferEventPayload::CompleteRejected { .. } => {}
            TransferEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = TransferStatus::Failed;
            }
            TransferEventPayload::FailRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;

    use super::{Transfer, TransferEventPayload, TransferId, TransferRequest, TransferStatus};

    #[test]
    fn request_initializes_state_and_records_event() {
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let amount = CurrencyAmount::new(100);
        let mut transfer = Transfer::default();

        transfer
            .request(TransferRequest {
                from_account_id,
                to_account_id,
                amount,
            })
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

        let result = transfer
            .request(TransferRequest {
                from_account_id: account_id,
                to_account_id: account_id,
                amount: CurrencyAmount::new(1),
            })
            .expect("same-account transfer should complete with a rejection event");

        assert!(matches!(
            result,
            super::TransferRequestResult::Rejected {
                reason: super::TransferRequestRejectionReason::SameAccount,
                ..
            }
        ));
        assert_eq!(
            transfer.status().expect("status should exist"),
            &TransferStatus::Rejected
        );
        assert_eq!(
            transfer.uncommitted_events()[0].payload().name(),
            TransferEventPayload::REQUEST_REJECTED
        );
    }

    #[test]
    fn request_errors_when_transfer_is_already_requested() {
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let amount = CurrencyAmount::new(100);
        let mut transfer = Transfer::default();

        transfer
            .request(TransferRequest {
                from_account_id,
                to_account_id,
                amount,
            })
            .expect("initial request should succeed");

        let error = transfer
            .request(TransferRequest {
                from_account_id,
                to_account_id,
                amount,
            })
            .expect_err("second request should be an unexpected processing error");

        assert!(matches!(error, super::TransferError::AlreadyRequested));
    }

    #[test]
    fn complete_updates_status() {
        let mut transfer = Transfer::default();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        transfer
            .request(TransferRequest {
                from_account_id,
                to_account_id,
                amount: CurrencyAmount::new(100),
            })
            .expect("request should succeed");

        transfer.complete().expect("complete should succeed");
        let duplicate_complete_result = transfer
            .complete()
            .expect("duplicate complete should complete with a rejection event");

        assert_eq!(
            transfer.status().expect("status should exist"),
            &TransferStatus::Completed
        );
        assert!(matches!(
            duplicate_complete_result,
            super::TransferCompleteResult::Rejected {
                reason: super::TransferCompleteRejectionReason::AlreadyCompleted
            }
        ));
    }

    #[test]
    fn fail_updates_status() {
        let mut transfer = Transfer::default();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        transfer
            .request(TransferRequest {
                from_account_id,
                to_account_id,
                amount: CurrencyAmount::new(100),
            })
            .expect("request should succeed");

        transfer
            .fail(super::TransferFailureReason::FundsReserveRejected)
            .expect("fail should succeed");
        let duplicate_fail_result = transfer
            .fail(super::TransferFailureReason::FundsReserveRejected)
            .expect("duplicate fail should complete with a rejection event");

        assert_eq!(
            transfer.status().expect("status should exist"),
            &TransferStatus::Failed
        );
        assert!(matches!(
            duplicate_fail_result,
            super::TransferFailResult::Rejected {
                reason: super::TransferFailRejectionReason::AlreadyFailed
            }
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
