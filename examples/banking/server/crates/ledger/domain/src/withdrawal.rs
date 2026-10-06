mod withdrawal_error;
mod withdrawal_event_payload;
mod withdrawal_event_payload_error;
mod withdrawal_failure_reason;
mod withdrawal_id;
mod withdrawal_note;
mod withdrawal_note_error;
mod withdrawal_state;
mod withdrawal_state_error;
mod withdrawal_status;

pub use withdrawal_error::WithdrawalError;
pub use withdrawal_event_payload::WithdrawalEventPayload;
pub use withdrawal_event_payload_error::WithdrawalEventPayloadError;
pub use withdrawal_failure_reason::WithdrawalFailureReason;
pub use withdrawal_id::WithdrawalId;
pub use withdrawal_note::WithdrawalNote;
pub use withdrawal_note_error::WithdrawalNoteError;
pub use withdrawal_state::WithdrawalState;
pub use withdrawal_state_error::WithdrawalStateError;
pub use withdrawal_status::WithdrawalStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::AccountId;
use crate::core::{CurrencyAmount, OnchainTransactionId, TokenOwnerAddress};
use crate::token_binding::TokenBindingId;

/// Represents the `Withdrawal` aggregate root.
#[aggregate(type = "withdrawal", error = WithdrawalError)]
pub struct Withdrawal {
    core: AggregateCore<WithdrawalId, WithdrawalState, WithdrawalEventPayload>,
}

impl Withdrawal {
    /// Returns the source account.
    pub fn account_id(&self) -> Result<&AccountId, WithdrawalError> {
        Ok(&self.state_required()?.account_id)
    }

    /// Returns the token binding selected for this withdrawal.
    pub fn token_binding_id(&self) -> Result<TokenBindingId, WithdrawalError> {
        Ok(self.state_required()?.token_binding_id)
    }

    /// Returns the destination token owner address.
    pub fn token_owner_address(&self) -> Result<&TokenOwnerAddress, WithdrawalError> {
        Ok(&self.state_required()?.token_owner_address)
    }

    /// Returns the withdrawal amount.
    pub fn amount(&self) -> Result<CurrencyAmount, WithdrawalError> {
        Ok(self.state_required()?.amount)
    }

    pub fn note(&self) -> Result<Option<&WithdrawalNote>, WithdrawalError> {
        Ok(self.state_required()?.note.as_ref())
    }

    /// Returns the verified settlement transaction when available.
    pub fn transaction_id(&self) -> Result<Option<&OnchainTransactionId>, WithdrawalError> {
        Ok(self.state_required()?.transaction_id.as_ref())
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<&WithdrawalStatus, WithdrawalError> {
        Ok(&self.state_required()?.status)
    }

    /// Requests a new withdrawal workflow.
    pub fn request(
        &mut self,
        account_id: AccountId,
        token_binding_id: TokenBindingId,
        token_owner_address: TokenOwnerAddress,
        amount: CurrencyAmount,
    ) -> Result<(), WithdrawalError> {
        if self.state().is_some() {
            return Err(WithdrawalError::AlreadyRequested);
        }

        if amount.is_zero() {
            return Err(WithdrawalError::ZeroAmount);
        }

        self.append_event(WithdrawalEventPayload::Requested {
            account_id,
            token_binding_id,
            token_owner_address,
            amount,
        })?;

        Ok(())
    }

    /// Sets or clears the note, including after success or failure.
    pub fn set_note(&mut self, note: Option<WithdrawalNote>) -> Result<(), WithdrawalError> {
        self.state_required()?;

        self.append_event(WithdrawalEventPayload::NoteSet { note })?;
        Ok(())
    }

    /// Records a successful external token settlement.
    pub fn record_settlement_executed(
        &mut self,
        transaction_id: OnchainTransactionId,
    ) -> Result<(), WithdrawalError> {
        match self.state_required()?.status {
            WithdrawalStatus::Pending => {}
            WithdrawalStatus::SettlementExecuted => {
                return Err(WithdrawalError::SettlementAlreadyExecuted);
            }
            WithdrawalStatus::Succeeded => {
                return Err(WithdrawalError::AlreadySucceeded);
            }
            WithdrawalStatus::Failed => {
                return Err(WithdrawalError::AlreadyFailed);
            }
            WithdrawalStatus::Rejected => {
                return Err(WithdrawalError::AlreadyRejected);
            }
        }

        self.append_event(WithdrawalEventPayload::SettlementExecuted { transaction_id })?;
        Ok(())
    }

    /// Records withdrawal success after internal accounting has been committed.
    pub fn succeed(&mut self) -> Result<(), WithdrawalError> {
        match self.state_required()?.status {
            WithdrawalStatus::SettlementExecuted => {}
            WithdrawalStatus::Pending => {
                return Err(WithdrawalError::SettlementNotExecuted);
            }
            WithdrawalStatus::Succeeded => {
                return Err(WithdrawalError::AlreadySucceeded);
            }
            WithdrawalStatus::Failed => {
                return Err(WithdrawalError::AlreadyFailed);
            }
            WithdrawalStatus::Rejected => {
                return Err(WithdrawalError::AlreadyRejected);
            }
        }

        self.append_event(WithdrawalEventPayload::Succeeded)?;
        Ok(())
    }

    /// Fails the withdrawal workflow.
    pub fn fail(&mut self, reason: WithdrawalFailureReason) -> Result<(), WithdrawalError> {
        match self.state_required()?.status {
            WithdrawalStatus::Pending | WithdrawalStatus::SettlementExecuted => {}
            WithdrawalStatus::Succeeded => {
                return Err(WithdrawalError::AlreadySucceeded);
            }
            WithdrawalStatus::Failed => {
                return Err(WithdrawalError::AlreadyFailed);
            }
            WithdrawalStatus::Rejected => {
                return Err(WithdrawalError::AlreadyRejected);
            }
        }

        self.append_event(WithdrawalEventPayload::Failed { reason })?;
        Ok(())
    }
}

impl AggregateApply<WithdrawalEventPayload, WithdrawalError> for Withdrawal {
    fn apply(&mut self, payload: &WithdrawalEventPayload) -> Result<(), WithdrawalError> {
        match payload {
            WithdrawalEventPayload::Requested {
                account_id,
                token_binding_id,
                token_owner_address,
                amount,
            } => self.set_state(Some(WithdrawalState {
                account_id: *account_id,
                token_binding_id: *token_binding_id,
                token_owner_address: *token_owner_address,
                amount: *amount,
                note: None,
                transaction_id: None,
                status: WithdrawalStatus::Pending,
            })),
            WithdrawalEventPayload::SettlementExecuted { transaction_id } => {
                let state = self.state_required_mut()?;
                state.transaction_id = Some(*transaction_id);
                state.status = WithdrawalStatus::SettlementExecuted;
            }
            WithdrawalEventPayload::NoteSet { note } => {
                self.state_required_mut()?.note = note.clone();
            }
            WithdrawalEventPayload::Succeeded => {
                self.state_required_mut()?.status = WithdrawalStatus::Succeeded;
            }
            WithdrawalEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = WithdrawalStatus::Failed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use appletheia::domain::{Aggregate, AggregateApply, EventPayload};

    use crate::account::AccountId;
    use crate::core::{
        CurrencyAmount, EvmTokenOwnerAddress, OnchainTransactionId, SolanaTransactionSignature,
        TokenOwnerAddress,
    };
    use crate::token_binding::TokenBindingId;

    use super::{Withdrawal, WithdrawalEventPayload, WithdrawalNote, WithdrawalStatus};

    #[test]
    fn records_an_executed_settlement() {
        let mut withdrawal = Withdrawal::new();
        withdrawal
            .request(
                AccountId::new(),
                TokenBindingId::new(),
                TokenOwnerAddress::Ethereum(
                    EvmTokenOwnerAddress::from_str("0x2222222222222222222222222222222222222222")
                        .expect("token owner address should be valid"),
                ),
                CurrencyAmount::new(100),
            )
            .expect("withdrawal request should succeed");
        withdrawal.core_mut().clear_uncommitted_events();
        let transaction_id = OnchainTransactionId::Solana(
            SolanaTransactionSignature::new(bs58::encode([1_u8; 64]).into_string())
                .expect("transaction signature should be valid"),
        );

        withdrawal
            .record_settlement_executed(transaction_id)
            .expect("settlement execution should be recorded");

        assert_eq!(
            withdrawal.status().expect("withdrawal state should exist"),
            &WithdrawalStatus::SettlementExecuted
        );
        assert_eq!(
            withdrawal.uncommitted_events()[0].payload().name(),
            WithdrawalEventPayload::SETTLEMENT_EXECUTED
        );
    }

    #[test]
    fn notes_can_be_set_and_cleared_after_success_and_replayed() {
        let mut withdrawal = Withdrawal::new();
        assert!(withdrawal.set_note(None).is_err());
        assert!(withdrawal.uncommitted_events().is_empty());
        withdrawal
            .request(
                AccountId::new(),
                TokenBindingId::new(),
                TokenOwnerAddress::Ethereum(
                    EvmTokenOwnerAddress::from_str("0x2222222222222222222222222222222222222222")
                        .unwrap(),
                ),
                CurrencyAmount::new(100),
            )
            .unwrap();
        assert_eq!(withdrawal.note().unwrap(), None);
        let note = WithdrawalNote::try_from("invoice 123").unwrap();
        withdrawal.set_note(Some(note.clone())).unwrap();
        assert_eq!(withdrawal.note().unwrap(), Some(&note));
        let transaction_id = OnchainTransactionId::Solana(
            SolanaTransactionSignature::new(bs58::encode([1_u8; 64]).into_string()).unwrap(),
        );
        withdrawal
            .record_settlement_executed(transaction_id)
            .unwrap();
        withdrawal.succeed().unwrap();
        let note = WithdrawalNote::try_from("invoice corrected").unwrap();
        withdrawal.set_note(Some(note.clone())).unwrap();
        assert_eq!(withdrawal.note().unwrap(), Some(&note));
        withdrawal.set_note(None).unwrap();
        let count = withdrawal.uncommitted_events().len();
        withdrawal.set_note(None).unwrap();
        assert_eq!(withdrawal.uncommitted_events().len(), count + 1);
        assert_eq!(withdrawal.note().unwrap(), None);

        let mut replayed = Withdrawal::new();
        for event in withdrawal.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert_eq!(replayed.state(), withdrawal.state());
    }
}
