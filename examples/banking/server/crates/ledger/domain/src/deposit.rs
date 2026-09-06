mod deposit_complete_rejection_reason;
mod deposit_complete_result;
mod deposit_error;
mod deposit_event_payload;
mod deposit_event_payload_error;
mod deposit_fail_rejection_reason;
mod deposit_fail_result;
mod deposit_failure_reason;
mod deposit_id;
mod deposit_note;
mod deposit_note_error;
mod deposit_request;
mod deposit_request_rejection_reason;
mod deposit_request_result;
mod deposit_settlement_verify_rejection_reason;
mod deposit_settlement_verify_result;
mod deposit_state;
mod deposit_state_error;
mod deposit_status;

pub use deposit_complete_rejection_reason::DepositCompleteRejectionReason;
pub use deposit_complete_result::DepositCompleteResult;
pub use deposit_error::DepositError;
pub use deposit_event_payload::DepositEventPayload;
pub use deposit_event_payload_error::DepositEventPayloadError;
pub use deposit_fail_rejection_reason::DepositFailRejectionReason;
pub use deposit_fail_result::DepositFailResult;
pub use deposit_failure_reason::DepositFailureReason;
pub use deposit_id::DepositId;
pub use deposit_note::DepositNote;
pub use deposit_note_error::DepositNoteError;
pub use deposit_request::DepositRequest;
pub use deposit_request_rejection_reason::DepositRequestRejectionReason;
pub use deposit_request_result::DepositRequestResult;
pub use deposit_settlement_verify_rejection_reason::DepositSettlementVerifyRejectionReason;
pub use deposit_settlement_verify_result::DepositSettlementVerifyResult;
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
        request: DepositRequest,
    ) -> Result<DepositRequestResult, DepositError> {
        if self.state().is_some() {
            return Err(DepositError::AlreadyRequested);
        }

        if request.amount.is_zero() {
            let reason = DepositRequestRejectionReason::ZeroAmount;
            self.reject_request(request, reason)?;
            return Ok(DepositRequestResult::Rejected { reason });
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

        Ok(DepositRequestResult::Requested)
    }

    /// Rejects a deposit request.
    pub fn reject_request(
        &mut self,
        _request: DepositRequest,
        reason: DepositRequestRejectionReason,
    ) -> Result<(), DepositError> {
        Err(DepositError::RequestRejected(reason))
    }

    /// Records the verified on-chain token settlement.
    pub fn record_settlement_verified(
        &mut self,
        transaction_id: OnchainTransactionId,
    ) -> Result<DepositSettlementVerifyResult, DepositError> {
        let state = self.state_required()?;
        match state.status {
            DepositStatus::Requested => {}
            DepositStatus::Rejected => {
                let reason = DepositSettlementVerifyRejectionReason::AlreadyRejected;
                self.reject_settlement_verify(transaction_id, reason)?;
                return Ok(DepositSettlementVerifyResult::Rejected { reason });
            }
            DepositStatus::SettlementVerified => {
                let reason = DepositSettlementVerifyRejectionReason::AlreadyVerified;
                self.reject_settlement_verify(transaction_id, reason)?;
                return Ok(DepositSettlementVerifyResult::Rejected { reason });
            }
            DepositStatus::Completed => {
                let reason = DepositSettlementVerifyRejectionReason::AlreadyCompleted;
                self.reject_settlement_verify(transaction_id, reason)?;
                return Ok(DepositSettlementVerifyResult::Rejected { reason });
            }
            DepositStatus::Failed => {
                let reason = DepositSettlementVerifyRejectionReason::AlreadyFailed;
                self.reject_settlement_verify(transaction_id, reason)?;
                return Ok(DepositSettlementVerifyResult::Rejected { reason });
            }
        }

        self.append_event(DepositEventPayload::SettlementVerified {
            account_id: state.account_id,
            amount: state.amount,
            transaction_id,
        })?;

        Ok(DepositSettlementVerifyResult::Verified)
    }

    /// Rejects recording an on-chain token settlement.
    pub fn reject_settlement_verify(
        &mut self,
        _transaction_id: OnchainTransactionId,
        reason: DepositSettlementVerifyRejectionReason,
    ) -> Result<(), DepositError> {
        Err(DepositError::SettlementVerifyRejected(reason))
    }

    /// Completes the deposit after internal accounting is applied.
    pub fn complete(&mut self) -> Result<DepositCompleteResult, DepositError> {
        match self.state_required()?.status {
            DepositStatus::Requested => return Err(DepositError::SettlementNotVerified),
            DepositStatus::Rejected => return Err(DepositError::SettlementNotVerified),
            DepositStatus::SettlementVerified => {}
            DepositStatus::Completed => {
                let reason = DepositCompleteRejectionReason::AlreadyCompleted;
                self.reject_complete(reason)?;
                return Ok(DepositCompleteResult::Rejected { reason });
            }
            DepositStatus::Failed => {
                let reason = DepositCompleteRejectionReason::AlreadyFailed;
                self.reject_complete(reason)?;
                return Ok(DepositCompleteResult::Rejected { reason });
            }
        }

        self.append_event(DepositEventPayload::Completed)?;
        Ok(DepositCompleteResult::Completed)
    }

    /// Rejects completing the deposit.
    pub fn reject_complete(
        &mut self,
        reason: DepositCompleteRejectionReason,
    ) -> Result<(), DepositError> {
        Err(DepositError::CompleteRejected(reason))
    }

    /// Fails the deposit workflow.
    pub fn fail(
        &mut self,
        reason: DepositFailureReason,
    ) -> Result<DepositFailResult, DepositError> {
        match self.state_required()?.status {
            DepositStatus::Requested => return Err(DepositError::SettlementNotVerified),
            DepositStatus::Rejected => return Err(DepositError::SettlementNotVerified),
            DepositStatus::SettlementVerified => {}
            DepositStatus::Completed => {
                let rejection_reason = DepositFailRejectionReason::AlreadyCompleted;
                self.reject_fail(rejection_reason)?;
                return Ok(DepositFailResult::Rejected {
                    reason: rejection_reason,
                });
            }
            DepositStatus::Failed => {
                let rejection_reason = DepositFailRejectionReason::AlreadyFailed;
                self.reject_fail(rejection_reason)?;
                return Ok(DepositFailResult::Rejected {
                    reason: rejection_reason,
                });
            }
        }

        self.append_event(DepositEventPayload::Failed { reason })?;
        Ok(DepositFailResult::Failed)
    }

    /// Rejects failing the deposit.
    pub fn reject_fail(&mut self, reason: DepositFailRejectionReason) -> Result<(), DepositError> {
        Err(DepositError::FailRejected(reason))
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
        CurrencyAmount, OnchainTransactionId, SolanaAccountAddress, SolanaTokenOwnerAddress,
        SolanaTransactionSignature, TokenOwnerAddress,
    };
    use crate::token_binding::TokenBindingId;

    use super::{
        Deposit, DepositEventPayload, DepositRequest, DepositSettlementVerifyResult, DepositStatus,
    };

    #[test]
    fn records_a_verified_settlement() {
        let mut deposit = Deposit::new();
        let token_owner_address = TokenOwnerAddress::Solana(SolanaTokenOwnerAddress::new(
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

        let result = deposit
            .record_settlement_verified(transaction_id)
            .expect("verified settlement should be recorded");

        assert_eq!(result, DepositSettlementVerifyResult::Verified);
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
