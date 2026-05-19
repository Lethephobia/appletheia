mod currency_issuance_complete_rejection_reason;
mod currency_issuance_complete_result;
mod currency_issuance_error;
mod currency_issuance_event_payload;
mod currency_issuance_event_payload_error;
mod currency_issuance_fail_rejection_reason;
mod currency_issuance_fail_result;
mod currency_issuance_failure_reason;
mod currency_issuance_id;
mod currency_issuance_issue_reject_result;
mod currency_issuance_issue_rejection_reason;
mod currency_issuance_issue_result;
mod currency_issuance_state;
mod currency_issuance_state_error;
mod currency_issuance_status;

pub use currency_issuance_complete_rejection_reason::CurrencyIssuanceCompleteRejectionReason;
pub use currency_issuance_complete_result::CurrencyIssuanceCompleteResult;
pub use currency_issuance_error::CurrencyIssuanceError;
pub use currency_issuance_event_payload::CurrencyIssuanceEventPayload;
pub use currency_issuance_event_payload_error::CurrencyIssuanceEventPayloadError;
pub use currency_issuance_fail_rejection_reason::CurrencyIssuanceFailRejectionReason;
pub use currency_issuance_fail_result::CurrencyIssuanceFailResult;
pub use currency_issuance_failure_reason::CurrencyIssuanceFailureReason;
pub use currency_issuance_id::CurrencyIssuanceId;
pub use currency_issuance_issue_reject_result::CurrencyIssuanceIssueRejectResult;
pub use currency_issuance_issue_rejection_reason::CurrencyIssuanceIssueRejectionReason;
pub use currency_issuance_issue_result::CurrencyIssuanceIssueResult;
pub use currency_issuance_state::CurrencyIssuanceState;
pub use currency_issuance_state_error::CurrencyIssuanceStateError;
pub use currency_issuance_status::CurrencyIssuanceStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::AccountId;
use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

/// Represents the `CurrencyIssuance` aggregate root.
#[aggregate(type = "currency_issuance", error = CurrencyIssuanceError)]
pub struct CurrencyIssuance {
    core: AggregateCore<CurrencyIssuanceState, CurrencyIssuanceEventPayload>,
}

impl CurrencyIssuance {
    /// Returns the issued currency.
    pub fn currency_id(&self) -> Result<&CurrencyId, CurrencyIssuanceError> {
        Ok(&self.state_required()?.currency_id)
    }

    /// Returns the destination account.
    pub fn destination_account_id(&self) -> Result<&AccountId, CurrencyIssuanceError> {
        Ok(&self.state_required()?.destination_account_id)
    }

    /// Returns the issuance amount.
    pub fn amount(&self) -> Result<&CurrencyAmount, CurrencyIssuanceError> {
        Ok(&self.state_required()?.amount)
    }

    /// Returns the current issuance status.
    pub fn status(&self) -> Result<&CurrencyIssuanceStatus, CurrencyIssuanceError> {
        Ok(&self.state_required()?.status)
    }

    /// Starts a new issuance workflow.
    pub fn issue(
        &mut self,
        currency_id: CurrencyId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<CurrencyIssuanceIssueResult, CurrencyIssuanceError> {
        if self.state().is_some() {
            return Err(CurrencyIssuanceError::AlreadyIssued);
        }

        if amount.is_zero() {
            let reject_result = self.reject_issue(
                currency_id,
                destination_account_id,
                amount,
                CurrencyIssuanceIssueRejectionReason::ZeroAmount,
            )?;
            let CurrencyIssuanceIssueRejectResult::Rejected { reason } = reject_result;
            return Ok(CurrencyIssuanceIssueResult::Rejected { reason });
        }

        let id = CurrencyIssuanceId::new();
        self.append_event(CurrencyIssuanceEventPayload::Issued {
            id,
            currency_id,
            destination_account_id,
            amount,
        })?;

        Ok(CurrencyIssuanceIssueResult::Issued)
    }

    /// Rejects a new issuance workflow.
    pub fn reject_issue(
        &mut self,
        currency_id: CurrencyId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
        reason: CurrencyIssuanceIssueRejectionReason,
    ) -> Result<CurrencyIssuanceIssueRejectResult, CurrencyIssuanceError> {
        let id = CurrencyIssuanceId::new();
        self.append_event(CurrencyIssuanceEventPayload::IssueRejected {
            id,
            currency_id,
            destination_account_id,
            amount,
            reason,
        })?;

        Ok(CurrencyIssuanceIssueRejectResult::Rejected { reason })
    }

    /// Marks the issuance completed.
    pub fn complete(&mut self) -> Result<CurrencyIssuanceCompleteResult, CurrencyIssuanceError> {
        match self.state_required()?.status {
            CurrencyIssuanceStatus::Pending => {}
            CurrencyIssuanceStatus::Completed => {
                let reason = CurrencyIssuanceCompleteRejectionReason::AlreadyCompleted;
                return self.reject_complete(reason);
            }
            CurrencyIssuanceStatus::Failed => {
                let reason = CurrencyIssuanceCompleteRejectionReason::AlreadyFailed;
                return self.reject_complete(reason);
            }
            CurrencyIssuanceStatus::Rejected => {
                let reason = CurrencyIssuanceCompleteRejectionReason::AlreadyRejected;
                return self.reject_complete(reason);
            }
        }

        self.append_event(CurrencyIssuanceEventPayload::Completed)?;
        Ok(CurrencyIssuanceCompleteResult::Completed)
    }

    /// Rejects completing an issuance workflow.
    pub fn reject_complete(
        &mut self,
        reason: CurrencyIssuanceCompleteRejectionReason,
    ) -> Result<CurrencyIssuanceCompleteResult, CurrencyIssuanceError> {
        self.append_event(CurrencyIssuanceEventPayload::CompleteRejected { reason })?;
        Ok(CurrencyIssuanceCompleteResult::Rejected { reason })
    }

    /// Marks the issuance failed.
    pub fn fail(
        &mut self,
        reason: CurrencyIssuanceFailureReason,
    ) -> Result<CurrencyIssuanceFailResult, CurrencyIssuanceError> {
        match self.state_required()?.status {
            CurrencyIssuanceStatus::Pending => {}
            CurrencyIssuanceStatus::Completed => {
                let reason = CurrencyIssuanceFailRejectionReason::AlreadyCompleted;
                return self.reject_fail(reason);
            }
            CurrencyIssuanceStatus::Failed => {
                let reason = CurrencyIssuanceFailRejectionReason::AlreadyFailed;
                return self.reject_fail(reason);
            }
            CurrencyIssuanceStatus::Rejected => {
                let reason = CurrencyIssuanceFailRejectionReason::AlreadyRejected;
                return self.reject_fail(reason);
            }
        }

        self.append_event(CurrencyIssuanceEventPayload::Failed { reason })?;
        Ok(CurrencyIssuanceFailResult::Failed)
    }

    /// Rejects failing an issuance workflow.
    pub fn reject_fail(
        &mut self,
        reason: CurrencyIssuanceFailRejectionReason,
    ) -> Result<CurrencyIssuanceFailResult, CurrencyIssuanceError> {
        self.append_event(CurrencyIssuanceEventPayload::FailRejected { reason })?;
        Ok(CurrencyIssuanceFailResult::Rejected { reason })
    }
}

impl AggregateApply<CurrencyIssuanceEventPayload, CurrencyIssuanceError> for CurrencyIssuance {
    fn apply(
        &mut self,
        payload: &CurrencyIssuanceEventPayload,
    ) -> Result<(), CurrencyIssuanceError> {
        match payload {
            CurrencyIssuanceEventPayload::Issued {
                id,
                currency_id,
                destination_account_id,
                amount,
            } => self.set_state(Some(CurrencyIssuanceState {
                id: *id,
                currency_id: *currency_id,
                destination_account_id: *destination_account_id,
                amount: *amount,
                status: CurrencyIssuanceStatus::Pending,
            })),
            CurrencyIssuanceEventPayload::IssueRejected {
                id,
                currency_id,
                destination_account_id,
                amount,
                ..
            } => self.set_state(Some(CurrencyIssuanceState {
                id: *id,
                currency_id: *currency_id,
                destination_account_id: *destination_account_id,
                amount: *amount,
                status: CurrencyIssuanceStatus::Rejected,
            })),
            CurrencyIssuanceEventPayload::Completed => {
                self.state_required_mut()?.status = CurrencyIssuanceStatus::Completed;
            }
            CurrencyIssuanceEventPayload::CompleteRejected { .. } => {}
            CurrencyIssuanceEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = CurrencyIssuanceStatus::Failed;
            }
            CurrencyIssuanceEventPayload::FailRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;
    use crate::currency::CurrencyId;

    use super::{
        CurrencyIssuance, CurrencyIssuanceEventPayload, CurrencyIssuanceId, CurrencyIssuanceStatus,
    };

    #[test]
    fn issue_initializes_state_and_records_event() {
        let currency_id = CurrencyId::new();
        let destination_account_id = AccountId::new();
        let amount = CurrencyAmount::new(100);
        let mut issuance = CurrencyIssuance::default();

        issuance
            .issue(currency_id, destination_account_id, amount)
            .expect("issue should succeed");

        assert_eq!(
            issuance.currency_id().expect("currency id should exist"),
            &currency_id
        );
        assert_eq!(
            issuance
                .destination_account_id()
                .expect("account id should exist"),
            &destination_account_id
        );
        assert_eq!(issuance.amount().expect("amount should exist"), &amount);
        assert_eq!(
            issuance.status().expect("status should exist"),
            &CurrencyIssuanceStatus::Pending
        );
        assert_eq!(
            issuance.uncommitted_events()[0].payload().name(),
            CurrencyIssuanceEventPayload::ISSUED
        );
    }

    #[test]
    fn issue_rejects_zero_amount() {
        let mut issuance = CurrencyIssuance::default();

        let result = issuance
            .issue(CurrencyId::new(), AccountId::new(), CurrencyAmount::zero())
            .expect("zero amount should complete with a rejection event");

        assert!(matches!(
            result,
            super::CurrencyIssuanceIssueResult::Rejected {
                reason: super::CurrencyIssuanceIssueRejectionReason::ZeroAmount
            }
        ));
        assert_eq!(
            issuance.status().expect("status should exist"),
            &CurrencyIssuanceStatus::Rejected
        );
    }

    #[test]
    fn issue_errors_when_issuance_is_already_issued() {
        let mut issuance = CurrencyIssuance::default();
        issuance
            .issue(
                CurrencyId::new(),
                AccountId::new(),
                CurrencyAmount::new(100),
            )
            .expect("issue should succeed");

        let error = issuance
            .issue(
                CurrencyId::new(),
                AccountId::new(),
                CurrencyAmount::new(100),
            )
            .expect_err("duplicate issue should fail");

        assert!(matches!(error, super::CurrencyIssuanceError::AlreadyIssued));
    }

    #[test]
    fn complete_updates_status() {
        let mut issuance = CurrencyIssuance::default();
        issuance
            .issue(
                CurrencyId::new(),
                AccountId::new(),
                CurrencyAmount::new(100),
            )
            .expect("issue should succeed");

        issuance.complete().expect("complete should succeed");

        assert_eq!(
            issuance.status().expect("status should exist"),
            &CurrencyIssuanceStatus::Completed
        );
    }

    #[test]
    fn fail_updates_status() {
        let mut issuance = CurrencyIssuance::default();
        issuance
            .issue(
                CurrencyId::new(),
                AccountId::new(),
                CurrencyAmount::new(100),
            )
            .expect("issue should succeed");

        issuance
            .fail(super::CurrencyIssuanceFailureReason::DepositRejected)
            .expect("fail should succeed");

        assert_eq!(
            issuance.status().expect("status should exist"),
            &CurrencyIssuanceStatus::Failed
        );
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let id = CurrencyIssuanceId::new();
        let currency_id = CurrencyId::new();
        let destination_account_id = AccountId::new();
        let issued = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            CurrencyIssuanceEventPayload::Issued {
                id,
                currency_id,
                destination_account_id,
                amount: CurrencyAmount::new(100),
            },
        );
        let completed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            CurrencyIssuanceEventPayload::Completed,
        );
        let mut issuance = CurrencyIssuance::default();

        issuance
            .replay_events(vec![issued, completed], None)
            .expect("events should replay");

        assert_eq!(
            issuance.currency_id().expect("currency id should exist"),
            &currency_id
        );
        assert_eq!(
            issuance.status().expect("status should exist"),
            &CurrencyIssuanceStatus::Completed
        );
        assert_eq!(issuance.version().value(), 2);
        assert!(issuance.uncommitted_events().is_empty());
    }
}
