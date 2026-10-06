mod deposit_error;
mod deposit_event_payload;
mod deposit_event_payload_error;
mod deposit_failure_reason;
mod deposit_id;
mod deposit_note;
mod deposit_note_error;
mod deposit_state;
mod deposit_state_error;
mod deposit_status;

pub use deposit_error::DepositError;
pub use deposit_event_payload::DepositEventPayload;
pub use deposit_event_payload_error::DepositEventPayloadError;
pub use deposit_failure_reason::DepositFailureReason;
pub use deposit_id::DepositId;
pub use deposit_note::DepositNote;
pub use deposit_note_error::DepositNoteError;
pub use deposit_state::DepositState;
pub use deposit_state_error::DepositStateError;
pub use deposit_status::DepositStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::AccountId;
use crate::core::{CurrencyAmount, OnchainTransactionId, TokenOwnerAddress};
use crate::token_binding::TokenBindingId;

/// Represents the `Deposit` aggregate root.
#[aggregate(type = "deposit", error = DepositError)]
pub struct Deposit {
    core: AggregateCore<DepositId, DepositState, DepositEventPayload>,
}

impl Deposit {
    /// Returns the destination account.
    pub fn account_id(&self) -> Result<&AccountId, DepositError> {
        Ok(&self.state_required()?.account_id)
    }

    /// Returns the token binding selected for this deposit.
    pub fn token_binding_id(&self) -> Result<TokenBindingId, DepositError> {
        Ok(self.state_required()?.token_binding_id)
    }

    /// Returns the owner of the external tokens deposited on-chain.
    pub fn token_owner_address(&self) -> Result<&TokenOwnerAddress, DepositError> {
        Ok(&self.state_required()?.token_owner_address)
    }

    /// Returns the deposit amount.
    pub fn amount(&self) -> Result<CurrencyAmount, DepositError> {
        Ok(self.state_required()?.amount)
    }

    pub fn note(&self) -> Result<Option<&DepositNote>, DepositError> {
        Ok(self.state_required()?.note.as_ref())
    }

    /// Returns the verified transaction identifier when settlement has succeeded.
    pub fn transaction_id(&self) -> Result<Option<&OnchainTransactionId>, DepositError> {
        Ok(self.state_required()?.transaction_id.as_ref())
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<&DepositStatus, DepositError> {
        Ok(&self.state_required()?.status)
    }

    /// Requests a deposit before its on-chain token settlement.
    pub fn request(
        &mut self,
        account_id: AccountId,
        token_binding_id: TokenBindingId,
        token_owner_address: TokenOwnerAddress,
        amount: CurrencyAmount,
    ) -> Result<(), DepositError> {
        if self.state().is_some() {
            return Err(DepositError::AlreadyRequested);
        }

        if amount.is_zero() {
            return Err(DepositError::ZeroAmount);
        }

        self.append_event(DepositEventPayload::Requested {
            account_id,
            token_binding_id,
            token_owner_address,
            amount,
        })?;

        Ok(())
    }

    /// Sets or clears the note, including after success or failure.
    pub fn set_note(&mut self, note: Option<DepositNote>) -> Result<(), DepositError> {
        self.state_required()?;

        self.append_event(DepositEventPayload::NoteSet { note })?;
        Ok(())
    }

    /// Records the verified on-chain token settlement.
    pub fn record_settlement_verified(
        &mut self,
        transaction_id: OnchainTransactionId,
    ) -> Result<(), DepositError> {
        let state = self.state_required()?;
        match state.status {
            DepositStatus::Requested => {}
            DepositStatus::Rejected => {
                return Err(DepositError::AlreadyRejected);
            }
            DepositStatus::SettlementVerified => {
                return Err(DepositError::SettlementAlreadyVerified);
            }
            DepositStatus::Succeeded => {
                return Err(DepositError::AlreadySucceeded);
            }
            DepositStatus::Failed => {
                return Err(DepositError::AlreadyFailed);
            }
        }

        self.append_event(DepositEventPayload::SettlementVerified {
            account_id: state.account_id,
            amount: state.amount,
            transaction_id,
        })?;

        Ok(())
    }

    /// Records deposit success after internal accounting has been applied.
    pub fn succeed(&mut self) -> Result<(), DepositError> {
        match self.state_required()?.status {
            DepositStatus::Requested => return Err(DepositError::SettlementNotVerified),
            DepositStatus::Rejected => return Err(DepositError::SettlementNotVerified),
            DepositStatus::SettlementVerified => {}
            DepositStatus::Succeeded => {
                return Err(DepositError::AlreadySucceeded);
            }
            DepositStatus::Failed => {
                return Err(DepositError::AlreadyFailed);
            }
        }

        self.append_event(DepositEventPayload::Succeeded)?;
        Ok(())
    }

    /// Fails the deposit workflow.
    pub fn fail(&mut self, reason: DepositFailureReason) -> Result<(), DepositError> {
        match self.state_required()?.status {
            DepositStatus::Requested => return Err(DepositError::SettlementNotVerified),
            DepositStatus::Rejected => return Err(DepositError::SettlementNotVerified),
            DepositStatus::SettlementVerified => {}
            DepositStatus::Succeeded => {
                return Err(DepositError::AlreadySucceeded);
            }
            DepositStatus::Failed => {
                return Err(DepositError::AlreadyFailed);
            }
        }

        self.append_event(DepositEventPayload::Failed { reason })?;
        Ok(())
    }
}

impl AggregateApply<DepositEventPayload, DepositError> for Deposit {
    fn apply(&mut self, payload: &DepositEventPayload) -> Result<(), DepositError> {
        match payload {
            DepositEventPayload::Requested {
                account_id,
                token_binding_id,
                token_owner_address,
                amount,
            } => self.set_state(Some(DepositState {
                account_id: *account_id,
                token_binding_id: *token_binding_id,
                token_owner_address: *token_owner_address,
                amount: *amount,
                note: None,
                transaction_id: None,
                status: DepositStatus::Requested,
            })),
            DepositEventPayload::SettlementVerified { transaction_id, .. } => {
                let state = self.state_required_mut()?;
                state.transaction_id = Some(*transaction_id);
                state.status = DepositStatus::SettlementVerified;
            }
            DepositEventPayload::NoteSet { note } => {
                self.state_required_mut()?.note = note.clone();
            }
            DepositEventPayload::Succeeded => {
                self.state_required_mut()?.status = DepositStatus::Succeeded;
            }
            DepositEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = DepositStatus::Failed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateApply, EventPayload};

    use crate::account::AccountId;
    use crate::core::{
        CurrencyAmount, OnchainTransactionId, SolanaAccountAddress, SolanaTokenAccountOwnerAddress,
        SolanaTransactionSignature, TokenOwnerAddress,
    };
    use crate::token_binding::TokenBindingId;

    use super::{Deposit, DepositEventPayload, DepositNote, DepositStatus};

    #[test]
    fn records_a_verified_settlement() {
        let mut deposit = Deposit::new();
        let token_owner_address = TokenOwnerAddress::Solana(SolanaTokenAccountOwnerAddress::new(
            SolanaAccountAddress::from_bytes([2; 32]),
        ));
        deposit
            .request(
                AccountId::new(),
                TokenBindingId::new(),
                token_owner_address,
                CurrencyAmount::new(100),
            )
            .expect("deposit request should succeed");
        deposit.core_mut().clear_uncommitted_events();
        let transaction_id = OnchainTransactionId::Solana(
            SolanaTransactionSignature::new(bs58::encode([1_u8; 64]).into_string())
                .expect("transaction signature should be valid"),
        );

        deposit
            .record_settlement_verified(transaction_id)
            .expect("verified settlement should be recorded");

        assert_eq!(
            deposit
                .token_owner_address()
                .expect("deposit state should exist"),
            &token_owner_address
        );
        assert_eq!(
            deposit.status().expect("deposit state should exist"),
            &DepositStatus::SettlementVerified
        );
        assert_eq!(
            deposit.uncommitted_events()[0].payload().name(),
            DepositEventPayload::SETTLEMENT_VERIFIED
        );
    }

    #[test]
    fn notes_can_be_set_and_cleared_after_success_and_replayed() {
        let mut deposit = Deposit::new();
        assert!(deposit.set_note(None).is_err());
        assert!(deposit.uncommitted_events().is_empty());
        deposit
            .request(
                AccountId::new(),
                TokenBindingId::new(),
                TokenOwnerAddress::Solana(SolanaTokenAccountOwnerAddress::new(
                    SolanaAccountAddress::from_bytes([2; 32]),
                )),
                CurrencyAmount::new(100),
            )
            .unwrap();
        assert_eq!(deposit.note().unwrap(), None);
        let note = DepositNote::try_from("invoice 123").unwrap();
        deposit.set_note(Some(note.clone())).unwrap();
        assert_eq!(deposit.note().unwrap(), Some(&note));
        let transaction_id = OnchainTransactionId::Solana(
            SolanaTransactionSignature::new(bs58::encode([1_u8; 64]).into_string()).unwrap(),
        );
        deposit.record_settlement_verified(transaction_id).unwrap();
        deposit.succeed().unwrap();
        let note = DepositNote::try_from("invoice corrected").unwrap();
        deposit.set_note(Some(note.clone())).unwrap();
        assert_eq!(deposit.note().unwrap(), Some(&note));
        deposit.set_note(None).unwrap();
        let count = deposit.uncommitted_events().len();
        deposit.set_note(None).unwrap();
        assert_eq!(deposit.uncommitted_events().len(), count + 1);
        assert_eq!(deposit.note().unwrap(), None);

        let mut replayed = Deposit::new();
        for event in deposit.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert_eq!(replayed.state(), deposit.state());
    }
}
