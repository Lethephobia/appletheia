mod withdrawal_complete_rejection_reason;
mod withdrawal_complete_result;
mod withdrawal_error;
mod withdrawal_event_payload;
mod withdrawal_event_payload_error;
mod withdrawal_fail_rejection_reason;
mod withdrawal_fail_result;
mod withdrawal_failure_reason;
mod withdrawal_id;
mod withdrawal_request;
mod withdrawal_request_rejection_reason;
mod withdrawal_request_result;
mod withdrawal_state;
mod withdrawal_state_error;
mod withdrawal_status;
mod withdrawal_token_transfer_rejection_reason;
mod withdrawal_token_transfer_result;

pub use withdrawal_complete_rejection_reason::WithdrawalCompleteRejectionReason;
pub use withdrawal_complete_result::WithdrawalCompleteResult;
pub use withdrawal_error::WithdrawalError;
pub use withdrawal_event_payload::WithdrawalEventPayload;
pub use withdrawal_event_payload_error::WithdrawalEventPayloadError;
pub use withdrawal_fail_rejection_reason::WithdrawalFailRejectionReason;
pub use withdrawal_fail_result::WithdrawalFailResult;
pub use withdrawal_failure_reason::WithdrawalFailureReason;
pub use withdrawal_id::WithdrawalId;
pub use withdrawal_request::WithdrawalRequest;
pub use withdrawal_request_rejection_reason::WithdrawalRequestRejectionReason;
pub use withdrawal_request_result::WithdrawalRequestResult;
pub use withdrawal_state::WithdrawalState;
pub use withdrawal_state_error::WithdrawalStateError;
pub use withdrawal_status::WithdrawalStatus;
pub use withdrawal_token_transfer_rejection_reason::WithdrawalTokenTransferRejectionReason;
pub use withdrawal_token_transfer_result::WithdrawalTokenTransferResult;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::AccountId;
use crate::core::{CurrencyAmount, TokenAccountOwnerAddress};
use crate::currency::CurrencyId;

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

    /// Returns the withdrawal currency.
    pub fn currency_id(&self) -> Result<&CurrencyId, WithdrawalError> {
        Ok(&self.state_required()?.currency_id)
    }

    /// Returns the token account owner address.
    pub fn token_account_owner_address(
        &self,
    ) -> Result<&TokenAccountOwnerAddress, WithdrawalError> {
        Ok(&self.state_required()?.token_account_owner_address)
    }

    /// Returns the withdrawal amount.
    pub fn amount(&self) -> Result<&CurrencyAmount, WithdrawalError> {
        Ok(&self.state_required()?.amount)
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<&WithdrawalStatus, WithdrawalError> {
        Ok(&self.state_required()?.status)
    }

    /// Requests a new withdrawal workflow.
    pub fn request(
        &mut self,
        request: WithdrawalRequest,
    ) -> Result<WithdrawalRequestResult, WithdrawalError> {
        if self.state().is_some() {
            return Err(WithdrawalError::AlreadyRequested);
        }

        if request.amount().is_zero() {
            let reason = WithdrawalRequestRejectionReason::ZeroAmount;
            self.reject_request(request, reason)?;
            return Ok(WithdrawalRequestResult::Rejected { reason });
        }

        let (account_id, currency_id, token_account_owner_address, amount) = request.into_parts();
        self.append_event(WithdrawalEventPayload::Requested {
            account_id,
            currency_id,
            token_account_owner_address,
            amount,
        })?;

        Ok(WithdrawalRequestResult::Requested)
    }

    /// Rejects a withdrawal request.
    pub fn reject_request(
        &mut self,
        request: WithdrawalRequest,
        reason: WithdrawalRequestRejectionReason,
    ) -> Result<(), WithdrawalError> {
        let (account_id, currency_id, token_account_owner_address, amount) = request.into_parts();
        self.append_event(WithdrawalEventPayload::RequestRejected {
            account_id,
            currency_id,
            token_account_owner_address,
            amount,
            reason,
        })?;
        Ok(())
    }

    /// Records a successful external token transfer.
    pub fn record_token_transfer(
        &mut self,
    ) -> Result<WithdrawalTokenTransferResult, WithdrawalError> {
        match self.state_required()?.status {
            WithdrawalStatus::Pending => {}
            WithdrawalStatus::TokenTransferred => {
                let reason = WithdrawalTokenTransferRejectionReason::AlreadyTokenTransferred;
                self.reject_token_transfer(reason)?;
                return Ok(WithdrawalTokenTransferResult::Rejected { reason });
            }
            WithdrawalStatus::Completed => {
                let reason = WithdrawalTokenTransferRejectionReason::AlreadyCompleted;
                self.reject_token_transfer(reason)?;
                return Ok(WithdrawalTokenTransferResult::Rejected { reason });
            }
            WithdrawalStatus::Failed => {
                let reason = WithdrawalTokenTransferRejectionReason::AlreadyFailed;
                self.reject_token_transfer(reason)?;
                return Ok(WithdrawalTokenTransferResult::Rejected { reason });
            }
            WithdrawalStatus::Rejected => {
                let reason = WithdrawalTokenTransferRejectionReason::AlreadyRejected;
                self.reject_token_transfer(reason)?;
                return Ok(WithdrawalTokenTransferResult::Rejected { reason });
            }
        }

        self.append_event(WithdrawalEventPayload::TokenTransferred)?;
        Ok(WithdrawalTokenTransferResult::TokenTransferred)
    }

    /// Rejects recording a successful external token transfer.
    pub fn reject_token_transfer(
        &mut self,
        reason: WithdrawalTokenTransferRejectionReason,
    ) -> Result<(), WithdrawalError> {
        self.append_event(WithdrawalEventPayload::TokenTransferRejected { reason })?;
        Ok(())
    }

    /// Completes the withdrawal after internal accounting is committed.
    pub fn complete(&mut self) -> Result<WithdrawalCompleteResult, WithdrawalError> {
        match self.state_required()?.status {
            WithdrawalStatus::TokenTransferred => {}
            WithdrawalStatus::Pending => {
                let reason = WithdrawalCompleteRejectionReason::TokenTransferNotRecorded;
                self.reject_complete(reason)?;
                return Ok(WithdrawalCompleteResult::Rejected { reason });
            }
            WithdrawalStatus::Completed => {
                let reason = WithdrawalCompleteRejectionReason::AlreadyCompleted;
                self.reject_complete(reason)?;
                return Ok(WithdrawalCompleteResult::Rejected { reason });
            }
            WithdrawalStatus::Failed => {
                let reason = WithdrawalCompleteRejectionReason::AlreadyFailed;
                self.reject_complete(reason)?;
                return Ok(WithdrawalCompleteResult::Rejected { reason });
            }
            WithdrawalStatus::Rejected => {
                let reason = WithdrawalCompleteRejectionReason::AlreadyRejected;
                self.reject_complete(reason)?;
                return Ok(WithdrawalCompleteResult::Rejected { reason });
            }
        }

        self.append_event(WithdrawalEventPayload::Completed)?;
        Ok(WithdrawalCompleteResult::Completed)
    }

    /// Rejects completing the withdrawal.
    pub fn reject_complete(
        &mut self,
        reason: WithdrawalCompleteRejectionReason,
    ) -> Result<(), WithdrawalError> {
        self.append_event(WithdrawalEventPayload::CompleteRejected { reason })?;
        Ok(())
    }

    /// Fails the withdrawal workflow.
    pub fn fail(
        &mut self,
        reason: WithdrawalFailureReason,
    ) -> Result<WithdrawalFailResult, WithdrawalError> {
        match self.state_required()?.status {
            WithdrawalStatus::Pending | WithdrawalStatus::TokenTransferred => {}
            WithdrawalStatus::Completed => {
                let rejection_reason = WithdrawalFailRejectionReason::AlreadyCompleted;
                self.reject_fail(rejection_reason)?;
                return Ok(WithdrawalFailResult::Rejected {
                    reason: rejection_reason,
                });
            }
            WithdrawalStatus::Failed => {
                let rejection_reason = WithdrawalFailRejectionReason::AlreadyFailed;
                self.reject_fail(rejection_reason)?;
                return Ok(WithdrawalFailResult::Rejected {
                    reason: rejection_reason,
                });
            }
            WithdrawalStatus::Rejected => {
                let rejection_reason = WithdrawalFailRejectionReason::AlreadyRejected;
                self.reject_fail(rejection_reason)?;
                return Ok(WithdrawalFailResult::Rejected {
                    reason: rejection_reason,
                });
            }
        }

        self.append_event(WithdrawalEventPayload::Failed { reason })?;
        Ok(WithdrawalFailResult::Failed)
    }

    /// Rejects failing the withdrawal.
    pub fn reject_fail(
        &mut self,
        reason: WithdrawalFailRejectionReason,
    ) -> Result<(), WithdrawalError> {
        self.append_event(WithdrawalEventPayload::FailRejected { reason })?;
        Ok(())
    }
}

impl AggregateApply<WithdrawalEventPayload, WithdrawalError> for Withdrawal {
    fn apply(&mut self, payload: &WithdrawalEventPayload) -> Result<(), WithdrawalError> {
        match payload {
            WithdrawalEventPayload::Requested {
                account_id,
                currency_id,
                token_account_owner_address,
                amount,
            } => self.set_state(Some(WithdrawalState {
                account_id: *account_id,
                currency_id: *currency_id,
                token_account_owner_address: token_account_owner_address.clone(),
                amount: *amount,
                status: WithdrawalStatus::Pending,
            })),
            WithdrawalEventPayload::RequestRejected {
                account_id,
                currency_id,
                token_account_owner_address,
                amount,
                ..
            } => self.set_state(Some(WithdrawalState {
                account_id: *account_id,
                currency_id: *currency_id,
                token_account_owner_address: token_account_owner_address.clone(),
                amount: *amount,
                status: WithdrawalStatus::Rejected,
            })),
            WithdrawalEventPayload::TokenTransferred => {
                self.state_required_mut()?.status = WithdrawalStatus::TokenTransferred;
            }
            WithdrawalEventPayload::TokenTransferRejected { .. } => {}
            WithdrawalEventPayload::Completed => {
                self.state_required_mut()?.status = WithdrawalStatus::Completed;
            }
            WithdrawalEventPayload::CompleteRejected { .. } => {}
            WithdrawalEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = WithdrawalStatus::Failed;
            }
            WithdrawalEventPayload::FailRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};

    use crate::{
        account::AccountId,
        core::{CurrencyAmount, TokenAccountOwnerAddress},
        currency::CurrencyId,
    };

    use super::{
        Withdrawal, WithdrawalCompleteResult, WithdrawalEventPayload, WithdrawalFailResult,
        WithdrawalFailureReason, WithdrawalRequest, WithdrawalRequestRejectionReason,
        WithdrawalRequestResult, WithdrawalStatus, WithdrawalTokenTransferRejectionReason,
        WithdrawalTokenTransferResult,
    };

    #[test]
    fn request_initializes_state_and_records_event() {
        let account_id = AccountId::new();
        let currency_id = CurrencyId::new();
        let token_account_owner_address = token_account_owner_address();
        let amount = CurrencyAmount::new(100);
        let mut withdrawal = Withdrawal::new();

        let result = withdrawal
            .request(WithdrawalRequest {
                account_id,
                currency_id,
                token_account_owner_address: token_account_owner_address.clone(),
                amount,
            })
            .expect("request should succeed");

        assert_eq!(result, WithdrawalRequestResult::Requested);
        assert_eq!(withdrawal.account_id().expect("account id"), &account_id);
        assert_eq!(withdrawal.currency_id().expect("currency id"), &currency_id);
        assert_eq!(
            withdrawal
                .token_account_owner_address()
                .expect("token account owner address"),
            &token_account_owner_address
        );
        assert_eq!(withdrawal.amount().expect("amount"), &amount);
        assert_eq!(
            withdrawal.status().expect("status"),
            &WithdrawalStatus::Pending
        );
        assert_eq!(
            withdrawal.uncommitted_events()[0].payload().name(),
            WithdrawalEventPayload::REQUESTED
        );
    }

    #[test]
    fn request_rejects_zero_amount() {
        let mut withdrawal = Withdrawal::new();

        let result = withdrawal
            .request(WithdrawalRequest {
                account_id: AccountId::new(),
                currency_id: CurrencyId::new(),
                token_account_owner_address: token_account_owner_address(),
                amount: CurrencyAmount::zero(),
            })
            .expect("request should succeed");

        assert_eq!(
            result,
            WithdrawalRequestResult::Rejected {
                reason: WithdrawalRequestRejectionReason::ZeroAmount,
            }
        );
        assert_eq!(
            withdrawal.status().expect("status"),
            &WithdrawalStatus::Rejected
        );
    }

    #[test]
    fn record_token_transfer_updates_status() {
        let mut withdrawal = requested_withdrawal();
        withdrawal.core_mut().clear_uncommitted_events();

        let result = withdrawal
            .record_token_transfer()
            .expect("record token transfer should succeed");

        assert_eq!(result, WithdrawalTokenTransferResult::TokenTransferred);
        assert_eq!(
            withdrawal.status().expect("status"),
            &WithdrawalStatus::TokenTransferred
        );
    }

    #[test]
    fn reject_token_transfer_records_rejection_event_without_changing_status() {
        let mut withdrawal = requested_withdrawal();
        withdrawal.core_mut().clear_uncommitted_events();

        withdrawal
            .reject_token_transfer(WithdrawalTokenTransferRejectionReason::AlreadyCompleted)
            .expect("reject token transfer should succeed");

        assert_eq!(
            withdrawal.status().expect("status"),
            &WithdrawalStatus::Pending
        );
        assert_eq!(
            withdrawal.uncommitted_events()[0].payload().name(),
            WithdrawalEventPayload::TOKEN_TRANSFER_REJECTED
        );
    }

    #[test]
    fn complete_requires_recorded_token_transfer() {
        let mut withdrawal = requested_withdrawal();
        withdrawal.core_mut().clear_uncommitted_events();

        let result = withdrawal.complete().expect("complete should succeed");

        assert_eq!(
            result,
            WithdrawalCompleteResult::Rejected {
                reason: super::WithdrawalCompleteRejectionReason::TokenTransferNotRecorded,
            }
        );
    }

    #[test]
    fn fail_marks_withdrawal_failed() {
        let mut withdrawal = requested_withdrawal();
        withdrawal.core_mut().clear_uncommitted_events();

        let result = withdrawal
            .fail(WithdrawalFailureReason::FundsReserveRejected)
            .expect("fail should succeed");

        assert_eq!(result, WithdrawalFailResult::Failed);
        assert_eq!(
            withdrawal.status().expect("status"),
            &WithdrawalStatus::Failed
        );
    }

    fn token_account_owner_address() -> TokenAccountOwnerAddress {
        TokenAccountOwnerAddress::try_from("11111111111111111111111111111111")
            .expect("token account owner address should be valid")
    }

    fn requested_withdrawal() -> Withdrawal {
        let mut withdrawal = Withdrawal::new();
        withdrawal
            .request(WithdrawalRequest {
                account_id: AccountId::new(),
                currency_id: CurrencyId::new(),
                token_account_owner_address: token_account_owner_address(),
                amount: CurrencyAmount::new(100),
            })
            .expect("request should succeed");
        withdrawal
    }
}
