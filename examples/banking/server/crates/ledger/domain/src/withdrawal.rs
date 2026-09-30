mod withdrawal_error;
mod withdrawal_event_payload;
mod withdrawal_event_payload_error;
mod withdrawal_failure_reason;
mod withdrawal_id;
mod withdrawal_note;
mod withdrawal_note_error;
mod withdrawal_request;
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
pub use withdrawal_request::WithdrawalRequest;
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
    pub fn request(&mut self, request: WithdrawalRequest) -> Result<(), WithdrawalError> {
        if self.state().is_some() {
            return Err(WithdrawalError::AlreadyRequested);
        }

        if request.amount().is_zero() {
            return Err(WithdrawalError::ZeroAmount);
        }

        let (account_id, token_binding_id, token_owner_address, amount, note) =
            request.into_parts();
        self.append_event(WithdrawalEventPayload::Requested {
            account_id,
            token_binding_id,
            token_owner_address,
            amount,
            note,
        })?;

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
            WithdrawalStatus::Completed => {
                return Err(WithdrawalError::AlreadyCompleted);
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

    /// Completes the withdrawal after internal accounting is committed.
    pub fn complete(&mut self) -> Result<(), WithdrawalError> {
        match self.state_required()?.status {
            WithdrawalStatus::SettlementExecuted => {}
            WithdrawalStatus::Pending => {
                return Err(WithdrawalError::SettlementNotExecuted);
            }
            WithdrawalStatus::Completed => {
                return Err(WithdrawalError::AlreadyCompleted);
            }
            WithdrawalStatus::Failed => {
                return Err(WithdrawalError::AlreadyFailed);
            }
            WithdrawalStatus::Rejected => {
                return Err(WithdrawalError::AlreadyRejected);
            }
        }

        self.append_event(WithdrawalEventPayload::Completed)?;
        Ok(())
    }

    /// Fails the withdrawal workflow.
    pub fn fail(&mut self, reason: WithdrawalFailureReason) -> Result<(), WithdrawalError> {
        match self.state_required()?.status {
            WithdrawalStatus::Pending | WithdrawalStatus::SettlementExecuted => {}
            WithdrawalStatus::Completed => {
                return Err(WithdrawalError::AlreadyCompleted);
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
                note,
            } => self.set_state(Some(WithdrawalState {
                account_id: *account_id,
                token_binding_id: *token_binding_id,
                token_owner_address: *token_owner_address,
                amount: *amount,
                note: note.clone(),
                transaction_id: None,
                status: WithdrawalStatus::Pending,
            })),
            WithdrawalEventPayload::SettlementExecuted { transaction_id } => {
                let state = self.state_required_mut()?;
                state.transaction_id = Some(*transaction_id);
                state.status = WithdrawalStatus::SettlementExecuted;
            }
            WithdrawalEventPayload::Completed => {
                self.state_required_mut()?.status = WithdrawalStatus::Completed;
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

    use appletheia::domain::{Aggregate, EventPayload};

    use crate::account::AccountId;
    use crate::core::{
        CurrencyAmount, EvmTokenOwnerAddress, OnchainTransactionId, SolanaTransactionSignature,
        TokenOwnerAddress,
    };
    use crate::token_binding::TokenBindingId;

    use super::{Withdrawal, WithdrawalEventPayload, WithdrawalRequest, WithdrawalStatus};

    #[test]
    fn records_an_executed_settlement() {
        let mut withdrawal = Withdrawal::new();
        withdrawal
            .request(WithdrawalRequest {
                account_id: AccountId::new(),
                token_binding_id: TokenBindingId::new(),
                token_owner_address: TokenOwnerAddress::Ethereum(
                    EvmTokenOwnerAddress::from_str("0x2222222222222222222222222222222222222222")
                        .expect("token owner address should be valid"),
                ),
                amount: CurrencyAmount::new(100),
                note: None,
            })
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
}
