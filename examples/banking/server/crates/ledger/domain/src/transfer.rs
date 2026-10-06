mod transfer_error;
mod transfer_event_payload;
mod transfer_event_payload_error;
mod transfer_failure_reason;
mod transfer_id;
mod transfer_note;
mod transfer_note_error;
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
            return Err(TransferError::SameSourceAndDestinationAccount);
        }

        if amount.is_zero() {
            return Err(TransferError::ZeroAmount);
        }

        self.append_event(TransferEventPayload::Requested {
            from_account_id,
            to_account_id,
            amount,
        })?;

        Ok(())
    }

    /// Sets or clears the note, including after success or failure.
    pub fn set_note(&mut self, note: Option<TransferNote>) -> Result<(), TransferError> {
        self.state_required()?;

        self.append_event(TransferEventPayload::NoteSet { note })?;
        Ok(())
    }

    /// Records transfer success after the source withdrawal has finished.
    pub fn succeed(&mut self) -> Result<(), TransferError> {
        match self.state_required()?.status {
            TransferStatus::Pending => {}
            TransferStatus::Succeeded => {
                return Err(TransferError::AlreadySucceeded);
            }
            TransferStatus::Failed => {
                return Err(TransferError::AlreadyFailed);
            }
            TransferStatus::Rejected => {
                return Err(TransferError::AlreadyRejected);
            }
        }

        self.append_event(TransferEventPayload::Succeeded)?;

        Ok(())
    }

    /// Fails the transfer.
    pub fn fail(&mut self, reason: TransferFailureReason) -> Result<(), TransferError> {
        match self.state_required()?.status {
            TransferStatus::Pending => {}
            TransferStatus::Succeeded => {
                return Err(TransferError::AlreadySucceeded);
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
            } => self.set_state(Some(TransferState {
                from_account_id: *from_account_id,
                to_account_id: *to_account_id,
                amount: *amount,
                note: None,
                status: TransferStatus::Pending,
            })),
            TransferEventPayload::NoteSet { note } => {
                self.state_required_mut()?.note = note.clone();
            }
            TransferEventPayload::Succeeded => {
                self.state_required_mut()?.status = TransferStatus::Succeeded;
            }
            TransferEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = TransferStatus::Failed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateApply};

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;

    use super::{Transfer, TransferNote};

    #[test]
    fn notes_can_be_set_and_cleared_after_success_and_replayed() {
        let mut transfer = Transfer::new();
        assert!(transfer.set_note(None).is_err());
        assert!(transfer.uncommitted_events().is_empty());
        transfer
            .request(AccountId::new(), AccountId::new(), CurrencyAmount::new(100))
            .unwrap();
        assert_eq!(transfer.note().unwrap(), None);
        let note = TransferNote::try_from("invoice 123").unwrap();
        transfer.set_note(Some(note.clone())).unwrap();
        assert_eq!(transfer.note().unwrap(), Some(&note));
        transfer.succeed().unwrap();
        let note = TransferNote::try_from("invoice corrected").unwrap();
        transfer.set_note(Some(note.clone())).unwrap();
        assert_eq!(transfer.note().unwrap(), Some(&note));
        transfer.set_note(None).unwrap();
        let count = transfer.uncommitted_events().len();
        transfer.set_note(None).unwrap();
        assert_eq!(transfer.uncommitted_events().len(), count + 1);
        assert_eq!(transfer.note().unwrap(), None);

        let mut replayed = Transfer::new();
        for event in transfer.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert_eq!(replayed.state(), transfer.state());
    }
}
