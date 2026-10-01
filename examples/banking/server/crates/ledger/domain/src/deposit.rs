mod deposit_error;
mod deposit_event_payload;
mod deposit_event_payload_error;
mod deposit_failure_reason;
mod deposit_id;
mod deposit_note;
mod deposit_note_error;
mod deposit_request;
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
pub use deposit_request::DepositRequest;
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
    pub fn request(&mut self, request: DepositRequest) -> Result<(), DepositError> {
        if self.state().is_some() {
            return Err(DepositError::AlreadyRequested);
        }

        if request.amount.is_zero() {
            return Err(DepositError::ZeroAmount);
        }

        let (account_id, token_binding_id, token_owner_address, amount, note) =
            request.into_parts();
        self.append_event(DepositEventPayload::Requested {
            account_id,
            token_binding_id,
            token_owner_address,
            amount,
            note,
        })?;

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
            DepositStatus::Completed => {
                return Err(DepositError::AlreadyCompleted);
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

    /// Completes the deposit after internal accounting is applied.
    pub fn complete(&mut self) -> Result<(), DepositError> {
        match self.state_required()?.status {
            DepositStatus::Requested => return Err(DepositError::SettlementNotVerified),
            DepositStatus::Rejected => return Err(DepositError::SettlementNotVerified),
            DepositStatus::SettlementVerified => {}
            DepositStatus::Completed => {
                return Err(DepositError::AlreadyCompleted);
            }
            DepositStatus::Failed => {
                return Err(DepositError::AlreadyFailed);
            }
        }

        self.append_event(DepositEventPayload::Completed)?;
        Ok(())
    }

    /// Fails the deposit workflow.
    pub fn fail(&mut self, reason: DepositFailureReason) -> Result<(), DepositError> {
        match self.state_required()?.status {
            DepositStatus::Requested => return Err(DepositError::SettlementNotVerified),
            DepositStatus::Rejected => return Err(DepositError::SettlementNotVerified),
            DepositStatus::SettlementVerified => {}
            DepositStatus::Completed => {
                return Err(DepositError::AlreadyCompleted);
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
                note,
            } => self.set_state(Some(DepositState {
                account_id: *account_id,
                token_binding_id: *token_binding_id,
                token_owner_address: *token_owner_address,
                amount: *amount,
                note: note.clone(),
                transaction_id: None,
                status: DepositStatus::Requested,
            })),
            DepositEventPayload::SettlementVerified { transaction_id, .. } => {
                let state = self.state_required_mut()?;
                state.transaction_id = Some(*transaction_id);
                state.status = DepositStatus::SettlementVerified;
            }
            DepositEventPayload::Completed => {
                self.state_required_mut()?.status = DepositStatus::Completed;
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
    use appletheia::domain::{Aggregate, EventPayload};

    use crate::account::AccountId;
    use crate::core::{
        CurrencyAmount, OnchainTransactionId, SolanaAccountAddress, SolanaTokenAccountOwnerAddress,
        SolanaTransactionSignature, TokenOwnerAddress,
    };
    use crate::token_binding::TokenBindingId;

    use super::{Deposit, DepositEventPayload, DepositRequest, DepositStatus};

    #[test]
    fn records_a_verified_settlement() {
        let mut deposit = Deposit::new();
        let token_owner_address = TokenOwnerAddress::Solana(SolanaTokenAccountOwnerAddress::new(
            SolanaAccountAddress::from_bytes([2; 32]),
        ));
        deposit
            .request(DepositRequest {
                account_id: AccountId::new(),
                token_binding_id: TokenBindingId::new(),
                token_owner_address,
                amount: CurrencyAmount::new(100),
                note: None,
            })
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
}
