mod deposit_complete_rejection_reason;
mod deposit_complete_result;
mod deposit_error;
mod deposit_event_payload;
mod deposit_event_payload_error;
mod deposit_fail_rejection_reason;
mod deposit_fail_result;
mod deposit_failure_reason;
mod deposit_id;
mod deposit_request;
mod deposit_request_rejection_reason;
mod deposit_request_result;
mod deposit_state;
mod deposit_state_error;
mod deposit_status;
mod deposit_token_transfer_record_rejection_reason;
mod deposit_token_transfer_result;

pub use deposit_complete_rejection_reason::DepositCompleteRejectionReason;
pub use deposit_complete_result::DepositCompleteResult;
pub use deposit_error::DepositError;
pub use deposit_event_payload::DepositEventPayload;
pub use deposit_event_payload_error::DepositEventPayloadError;
pub use deposit_fail_rejection_reason::DepositFailRejectionReason;
pub use deposit_fail_result::DepositFailResult;
pub use deposit_failure_reason::DepositFailureReason;
pub use deposit_id::DepositId;
pub use deposit_request::DepositRequest;
pub use deposit_request_rejection_reason::DepositRequestRejectionReason;
pub use deposit_request_result::DepositRequestResult;
pub use deposit_state::DepositState;
pub use deposit_state_error::DepositStateError;
pub use deposit_status::DepositStatus;
pub use deposit_token_transfer_record_rejection_reason::DepositTokenTransferRecordRejectionReason;
pub use deposit_token_transfer_result::DepositTokenTransferResult;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::AccountId;
use crate::core::{CurrencyAmount, TokenAccountOwnerAddress};
use crate::currency::CurrencyId;

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

    /// Returns the deposit currency.
    pub fn currency_id(&self) -> Result<&CurrencyId, DepositError> {
        Ok(&self.state_required()?.currency_id)
    }

    /// Returns the token account owner address.
    pub fn token_account_owner_address(&self) -> Result<&TokenAccountOwnerAddress, DepositError> {
        Ok(&self.state_required()?.token_account_owner_address)
    }

    /// Returns the deposit amount.
    pub fn amount(&self) -> Result<&CurrencyAmount, DepositError> {
        Ok(&self.state_required()?.amount)
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<&DepositStatus, DepositError> {
        Ok(&self.state_required()?.status)
    }

    /// Requests a deposit before its on-chain token transfer.
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

        let (account_id, currency_id, token_account_owner_address, amount) = request.into_parts();
        self.append_event(DepositEventPayload::Requested {
            account_id,
            currency_id,
            token_account_owner_address,
            amount,
        })?;

        Ok(DepositRequestResult::Requested)
    }

    /// Rejects a deposit request.
    pub fn reject_request(
        &mut self,
        request: DepositRequest,
        reason: DepositRequestRejectionReason,
    ) -> Result<(), DepositError> {
        let (account_id, currency_id, token_account_owner_address, amount) = request.into_parts();
        self.append_event(DepositEventPayload::RequestRejected {
            account_id,
            currency_id,
            token_account_owner_address,
            amount,
            reason,
        })?;
        Ok(())
    }

    /// Records the verified on-chain token transfer.
    pub fn record_token_transfer(&mut self) -> Result<DepositTokenTransferResult, DepositError> {
        let state = self.state_required()?;
        match state.status {
            DepositStatus::Requested => {}
            DepositStatus::Rejected => {
                let reason = DepositTokenTransferRecordRejectionReason::AlreadyRejected;
                self.reject_token_transfer_record(reason)?;
                return Ok(DepositTokenTransferResult::Rejected { reason });
            }
            DepositStatus::TokenTransferred => {
                let reason = DepositTokenTransferRecordRejectionReason::AlreadyTokenTransferred;
                self.reject_token_transfer_record(reason)?;
                return Ok(DepositTokenTransferResult::Rejected { reason });
            }
            DepositStatus::Completed => {
                let reason = DepositTokenTransferRecordRejectionReason::AlreadyCompleted;
                self.reject_token_transfer_record(reason)?;
                return Ok(DepositTokenTransferResult::Rejected { reason });
            }
            DepositStatus::Failed => {
                let reason = DepositTokenTransferRecordRejectionReason::AlreadyFailed;
                self.reject_token_transfer_record(reason)?;
                return Ok(DepositTokenTransferResult::Rejected { reason });
            }
        }

        self.append_event(DepositEventPayload::TokenTransferred {
            account_id: state.account_id,
            amount: state.amount,
        })?;

        Ok(DepositTokenTransferResult::TokenTransferred)
    }

    /// Rejects recording an on-chain token transfer.
    pub fn reject_token_transfer_record(
        &mut self,
        reason: DepositTokenTransferRecordRejectionReason,
    ) -> Result<(), DepositError> {
        self.append_event(DepositEventPayload::TokenTransferRecordRejected { reason })?;
        Ok(())
    }

    /// Completes the deposit after internal accounting is applied.
    pub fn complete(&mut self) -> Result<DepositCompleteResult, DepositError> {
        match self.state_required()?.status {
            DepositStatus::Requested => return Err(DepositError::TokenTransferNotRecorded),
            DepositStatus::Rejected => return Err(DepositError::TokenTransferNotRecorded),
            DepositStatus::TokenTransferred => {}
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
        self.append_event(DepositEventPayload::CompleteRejected { reason })?;
        Ok(())
    }

    /// Fails the deposit workflow.
    pub fn fail(
        &mut self,
        reason: DepositFailureReason,
    ) -> Result<DepositFailResult, DepositError> {
        match self.state_required()?.status {
            DepositStatus::Requested => return Err(DepositError::TokenTransferNotRecorded),
            DepositStatus::Rejected => return Err(DepositError::TokenTransferNotRecorded),
            DepositStatus::TokenTransferred => {}
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
        self.append_event(DepositEventPayload::FailRejected { reason })?;
        Ok(())
    }
}

impl AggregateApply<DepositEventPayload, DepositError> for Deposit {
    fn apply(&mut self, payload: &DepositEventPayload) -> Result<(), DepositError> {
        match payload {
            DepositEventPayload::Requested {
                account_id,
                currency_id,
                token_account_owner_address,
                amount,
            } => self.set_state(Some(DepositState {
                account_id: *account_id,
                currency_id: *currency_id,
                token_account_owner_address: token_account_owner_address.clone(),
                amount: *amount,
                status: DepositStatus::Requested,
            })),
            DepositEventPayload::RequestRejected {
                account_id,
                currency_id,
                token_account_owner_address,
                amount,
                ..
            } => self.set_state(Some(DepositState {
                account_id: *account_id,
                currency_id: *currency_id,
                token_account_owner_address: token_account_owner_address.clone(),
                amount: *amount,
                status: DepositStatus::Rejected,
            })),
            DepositEventPayload::TokenTransferred { .. } => {
                self.state_required_mut()?.status = DepositStatus::TokenTransferred;
            }
            DepositEventPayload::TokenTransferRecordRejected { .. } => {}
            DepositEventPayload::Completed => {
                self.state_required_mut()?.status = DepositStatus::Completed;
            }
            DepositEventPayload::CompleteRejected { .. } => {}
            DepositEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = DepositStatus::Failed;
            }
            DepositEventPayload::FailRejected { .. } => {}
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
        Deposit, DepositCompleteResult, DepositEventPayload, DepositRequest,
        DepositRequestRejectionReason, DepositRequestResult, DepositStatus,
        DepositTokenTransferResult,
    };

    #[test]
    fn request_initializes_state_and_records_event() {
        let account_id = AccountId::new();
        let currency_id = CurrencyId::new();
        let token_account_owner_address = token_account_owner_address();
        let amount = CurrencyAmount::new(100);
        let mut deposit = Deposit::new();
        let result = deposit
            .request(DepositRequest {
                account_id,
                currency_id,
                token_account_owner_address: token_account_owner_address.clone(),
                amount,
            })
            .expect("deposit should be requested");
        assert_eq!(result, DepositRequestResult::Requested);

        assert!(matches!(
            deposit.uncommitted_events()[0].payload(),
            DepositEventPayload::Requested { .. }
        ));
        assert_eq!(deposit.account_id().expect("account id"), &account_id);
        assert_eq!(deposit.currency_id().expect("currency id"), &currency_id);
        assert_eq!(
            deposit
                .token_account_owner_address()
                .expect("token account owner address"),
            &token_account_owner_address
        );
        assert_eq!(deposit.amount().expect("amount"), &amount);
        assert_eq!(deposit.status().expect("status"), &DepositStatus::Requested);
        assert_eq!(
            deposit.uncommitted_events()[0].payload().name(),
            DepositEventPayload::REQUESTED
        );
    }

    #[test]
    fn record_token_transfer_updates_requested_deposit() {
        let mut deposit = requested_deposit();

        let result = deposit
            .record_token_transfer()
            .expect("token transfer should be recorded");

        assert_eq!(result, DepositTokenTransferResult::TokenTransferred);
        assert_eq!(
            deposit.status().expect("status"),
            &DepositStatus::TokenTransferred
        );
        assert_eq!(
            deposit.uncommitted_events()[1].payload().name(),
            DepositEventPayload::TOKEN_TRANSFERRED
        );
    }

    #[test]
    fn request_rejects_zero_amount() {
        let mut deposit = Deposit::new();

        let result = deposit
            .request(DepositRequest {
                account_id: AccountId::new(),
                currency_id: CurrencyId::new(),
                token_account_owner_address: token_account_owner_address(),
                amount: CurrencyAmount::zero(),
            })
            .expect("zero amount should be rejected");

        assert!(matches!(
            result,
            DepositRequestResult::Rejected {
                reason: DepositRequestRejectionReason::ZeroAmount,
                ..
            }
        ));
        assert_eq!(deposit.status().expect("status"), &DepositStatus::Rejected);
        assert!(matches!(
            deposit.uncommitted_events()[0].payload(),
            DepositEventPayload::RequestRejected {
                reason: DepositRequestRejectionReason::ZeroAmount,
                ..
            }
        ));
    }

    #[test]
    fn complete_succeeds_after_token_transfer_is_recorded() {
        let mut deposit = requested_deposit();
        deposit
            .record_token_transfer()
            .expect("token transfer should be recorded");

        let result = deposit.complete().expect("complete should succeed");

        assert_eq!(result, DepositCompleteResult::Completed);
        assert_eq!(deposit.status().expect("status"), &DepositStatus::Completed);
    }

    fn requested_deposit() -> Deposit {
        let mut deposit = Deposit::new();
        deposit
            .request(DepositRequest {
                account_id: AccountId::new(),
                currency_id: CurrencyId::new(),
                token_account_owner_address: token_account_owner_address(),
                amount: CurrencyAmount::new(100),
            })
            .expect("deposit should be requested");
        deposit
    }

    fn token_account_owner_address() -> TokenAccountOwnerAddress {
        TokenAccountOwnerAddress::try_from("11111111111111111111111111111111")
            .expect("token account owner address should be valid")
    }
}
