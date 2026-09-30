mod transfer_error;
mod transfer_event_payload;
mod transfer_event_payload_error;
mod transfer_failure_reason;
mod transfer_id;
mod transfer_note;
mod transfer_note_error;
mod transfer_request;
mod transfer_state;
mod transfer_state_error;
mod transfer_status;

pub use transfer_error::TransferError;
pub use transfer_event_payload::TransferEventPayload;
pub use transfer_event_payload_error::TransferEventPayloadError;
pub use transfer_failure_reason::TransferFailureReason;
pub use transfer_id::TransferId;
pub use transfer_note::TransferNote;
pub use transfer_note_error::TransferNoteError;
pub use transfer_request::TransferRequest;
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
    core: AggregateCore<TransferId, TransferState, TransferEventPayload>,
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
    pub fn amount(&self) -> Result<CurrencyAmount, TransferError> {
        Ok(self.state_required()?.amount)
    }

    pub fn note(&self) -> Result<Option<&TransferNote>, TransferError> {
        Ok(self.state_required()?.note.as_ref())
    }

    /// Returns the current transfer status.
    pub fn status(&self) -> Result<&TransferStatus, TransferError> {
        Ok(&self.state_required()?.status)
    }

    /// Requests a new transfer.
    pub fn request(&mut self, request: TransferRequest) -> Result<(), TransferError> {
        if self.state().is_some() {
            return Err(TransferError::AlreadyRequested);
        }

        if request.is_same_account() {
            return Err(TransferError::SameSourceAndDestinationAccount);
        }

        if request.amount().is_zero() {
            return Err(TransferError::ZeroAmount);
        }

        let (from_account_id, to_account_id, amount, note) = request.into_parts();
        self.append_event(TransferEventPayload::Requested {
            from_account_id,
            to_account_id,
            amount,
            note,
        })?;

        Ok(())
    }

    /// Completes the transfer.
    pub fn complete(&mut self) -> Result<(), TransferError> {
        match self.state_required()?.status {
            TransferStatus::Pending => {}
            TransferStatus::Completed => {
                return Err(TransferError::AlreadyCompleted);
            }
            TransferStatus::Failed => {
                return Err(TransferError::AlreadyFailed);
            }
            TransferStatus::Rejected => {
                return Err(TransferError::AlreadyRejected);
            }
        }

        self.append_event(TransferEventPayload::Completed)?;

        Ok(())
    }

    /// Fails the transfer.
    pub fn fail(&mut self, reason: TransferFailureReason) -> Result<(), TransferError> {
        match self.state_required()?.status {
            TransferStatus::Pending => {}
            TransferStatus::Completed => {
                return Err(TransferError::AlreadyCompleted);
            }
            TransferStatus::Failed => {
                return Err(TransferError::AlreadyFailed);
            }
            TransferStatus::Rejected => {
                return Err(TransferError::AlreadyRejected);
            }
        }

        self.append_event(TransferEventPayload::Failed { reason })?;

        Ok(())
    }
}

impl AggregateApply<TransferEventPayload, TransferError> for Transfer {
    fn apply(&mut self, payload: &TransferEventPayload) -> Result<(), TransferError> {
        match payload {
            TransferEventPayload::Requested {
                from_account_id,
                to_account_id,
                amount,
                note,
            } => self.set_state(Some(TransferState {
                from_account_id: *from_account_id,
                to_account_id: *to_account_id,
                amount: *amount,
                note: note.clone(),
                status: TransferStatus::Pending,
            })),
            TransferEventPayload::Completed => {
                self.state_required_mut()?.status = TransferStatus::Completed;
            }
            TransferEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = TransferStatus::Failed;
            }
        }

        Ok(())
    }
}
