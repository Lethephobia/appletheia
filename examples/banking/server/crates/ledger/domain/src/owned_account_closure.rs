mod owned_account_closure_complete_rejection_reason;
mod owned_account_closure_complete_result;
mod owned_account_closure_error;
mod owned_account_closure_event_payload;
mod owned_account_closure_event_payload_error;
mod owned_account_closure_fail_rejection_reason;
mod owned_account_closure_fail_result;
mod owned_account_closure_failure_reason;
mod owned_account_closure_id;
mod owned_account_closure_page_load_rejection_reason;
mod owned_account_closure_page_load_result;
mod owned_account_closure_record_rejection_reason;
mod owned_account_closure_record_result;
mod owned_account_closure_state;
mod owned_account_closure_state_error;
mod owned_account_closure_status;

pub use owned_account_closure_complete_rejection_reason::OwnedAccountClosureCompleteRejectionReason;
pub use owned_account_closure_complete_result::OwnedAccountClosureCompleteResult;
pub use owned_account_closure_error::OwnedAccountClosureError;
pub use owned_account_closure_event_payload::OwnedAccountClosureEventPayload;
pub use owned_account_closure_event_payload_error::OwnedAccountClosureEventPayloadError;
pub use owned_account_closure_fail_rejection_reason::OwnedAccountClosureFailRejectionReason;
pub use owned_account_closure_fail_result::OwnedAccountClosureFailResult;
pub use owned_account_closure_failure_reason::OwnedAccountClosureFailureReason;
pub use owned_account_closure_id::OwnedAccountClosureId;
pub use owned_account_closure_page_load_rejection_reason::OwnedAccountClosurePageLoadRejectionReason;
pub use owned_account_closure_page_load_result::OwnedAccountClosurePageLoadResult;
pub use owned_account_closure_record_rejection_reason::OwnedAccountClosureRecordRejectionReason;
pub use owned_account_closure_record_result::OwnedAccountClosureRecordResult;
pub use owned_account_closure_state::OwnedAccountClosureState;
pub use owned_account_closure_state_error::OwnedAccountClosureStateError;
pub use owned_account_closure_status::OwnedAccountClosureStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::{AccountCloseRejectionReason, AccountId, AccountOwner};

/// Represents the `OwnedAccountClosure` process aggregate.
#[aggregate(type = "owned_account_closure", error = OwnedAccountClosureError)]
pub struct OwnedAccountClosure {
    core: AggregateCore<OwnedAccountClosureState, OwnedAccountClosureEventPayload>,
}

impl OwnedAccountClosure {
    /// Returns the owner whose accounts are being closed.
    pub fn owner(&self) -> Result<AccountOwner, OwnedAccountClosureError> {
        Ok(self.state_required()?.owner)
    }

    /// Starts a workflow that closes every account owned by the owner.
    pub fn request(&mut self, owner: AccountOwner) -> Result<(), OwnedAccountClosureError> {
        if self.state().is_some() {
            return Err(OwnedAccountClosureError::AlreadyRequested);
        }

        let id = OwnedAccountClosureId::new();
        self.append_event(OwnedAccountClosureEventPayload::Requested { id, owner })?;
        Ok(())
    }

    /// Records one page of accounts owned by the workflow owner.
    pub fn load_page(
        &mut self,
        account_ids: Vec<AccountId>,
        next_cursor: Option<AccountId>,
    ) -> Result<OwnedAccountClosurePageLoadResult, OwnedAccountClosureError> {
        if self.state_required()?.status.is_terminal() {
            let reason = OwnedAccountClosurePageLoadRejectionReason::AlreadyTerminal;
            self.append_event(OwnedAccountClosureEventPayload::PageLoadRejected { reason })?;
            return Ok(OwnedAccountClosurePageLoadResult::Rejected { reason });
        }

        self.append_event(OwnedAccountClosureEventPayload::PageLoaded {
            account_ids,
            next_cursor,
        })?;
        Ok(OwnedAccountClosurePageLoadResult::Loaded)
    }

    /// Records a successful account close result.
    pub fn record_account_close(
        &mut self,
        account_id: AccountId,
    ) -> Result<OwnedAccountClosureRecordResult, OwnedAccountClosureError> {
        let state = self.state_required()?;
        if state.status.is_terminal() {
            let reason = OwnedAccountClosureRecordRejectionReason::AlreadyTerminal;
            self.append_event(
                OwnedAccountClosureEventPayload::AccountCloseRecordRejected { account_id, reason },
            )?;
            return Ok(OwnedAccountClosureRecordResult::Rejected { reason });
        }

        self.append_event(OwnedAccountClosureEventPayload::AccountCloseRecorded { account_id })?;
        Ok(OwnedAccountClosureRecordResult::Recorded)
    }

    /// Records a rejected account close result.
    pub fn record_account_close_rejection(
        &mut self,
        account_id: AccountId,
        reason: AccountCloseRejectionReason,
    ) -> Result<OwnedAccountClosureRecordResult, OwnedAccountClosureError> {
        let state = self.state_required()?;
        if state.status.is_terminal() {
            let record_rejection_reason = OwnedAccountClosureRecordRejectionReason::AlreadyTerminal;
            self.append_event(
                OwnedAccountClosureEventPayload::AccountCloseRejectionRecordRejected {
                    account_id,
                    reason: record_rejection_reason,
                },
            )?;
            return Ok(OwnedAccountClosureRecordResult::Rejected {
                reason: record_rejection_reason,
            });
        }

        self.append_event(
            OwnedAccountClosureEventPayload::AccountCloseRejectionRecorded { account_id, reason },
        )?;
        Ok(OwnedAccountClosureRecordResult::Recorded)
    }

    /// Marks the workflow completed.
    pub fn complete(
        &mut self,
    ) -> Result<OwnedAccountClosureCompleteResult, OwnedAccountClosureError> {
        let state = self.state_required()?;
        match state.status {
            OwnedAccountClosureStatus::Requested => {
                let reason = OwnedAccountClosureCompleteRejectionReason::NotInProgress;
                self.append_event(OwnedAccountClosureEventPayload::CompleteRejected { reason })?;
                return Ok(OwnedAccountClosureCompleteResult::Rejected { reason });
            }
            OwnedAccountClosureStatus::InProgress => {}
            OwnedAccountClosureStatus::Completed => {
                let reason = OwnedAccountClosureCompleteRejectionReason::AlreadyCompleted;
                self.append_event(OwnedAccountClosureEventPayload::CompleteRejected { reason })?;
                return Ok(OwnedAccountClosureCompleteResult::Rejected { reason });
            }
            OwnedAccountClosureStatus::Failed => {
                let reason = OwnedAccountClosureCompleteRejectionReason::AlreadyFailed;
                self.append_event(OwnedAccountClosureEventPayload::CompleteRejected { reason })?;
                return Ok(OwnedAccountClosureCompleteResult::Rejected { reason });
            }
        }

        let state = self.state_required()?;
        if state.rejected_account_count() > 0 {
            let reason = OwnedAccountClosureCompleteRejectionReason::AccountCloseRejected;
            self.append_event(OwnedAccountClosureEventPayload::CompleteRejected { reason })?;
            return Ok(OwnedAccountClosureCompleteResult::Rejected { reason });
        }

        let closed_account_count = state.closed_account_count();
        self.append_event(OwnedAccountClosureEventPayload::Completed {
            closed_account_count,
        })?;
        Ok(OwnedAccountClosureCompleteResult::Completed)
    }

    /// Marks the workflow failed after all account close attempts were recorded.
    pub fn fail(
        &mut self,
        reason: OwnedAccountClosureFailureReason,
    ) -> Result<OwnedAccountClosureFailResult, OwnedAccountClosureError> {
        match self.state_required()?.status {
            OwnedAccountClosureStatus::Requested | OwnedAccountClosureStatus::InProgress => {}
            OwnedAccountClosureStatus::Completed => {
                let reason = OwnedAccountClosureFailRejectionReason::AlreadyCompleted;
                self.append_event(OwnedAccountClosureEventPayload::FailRejected { reason })?;
                return Ok(OwnedAccountClosureFailResult::Rejected { reason });
            }
            OwnedAccountClosureStatus::Failed => {
                let reason = OwnedAccountClosureFailRejectionReason::AlreadyFailed;
                self.append_event(OwnedAccountClosureEventPayload::FailRejected { reason })?;
                return Ok(OwnedAccountClosureFailResult::Rejected { reason });
            }
        }

        let state = self.state_required()?;
        self.append_event(OwnedAccountClosureEventPayload::Failed {
            closed_account_count: state.closed_account_count(),
            rejected_account_count: state.rejected_account_count(),
            reason,
        })?;
        Ok(OwnedAccountClosureFailResult::Failed)
    }
}

impl AggregateApply<OwnedAccountClosureEventPayload, OwnedAccountClosureError>
    for OwnedAccountClosure
{
    fn apply(
        &mut self,
        payload: &OwnedAccountClosureEventPayload,
    ) -> Result<(), OwnedAccountClosureError> {
        match payload {
            OwnedAccountClosureEventPayload::Requested { id, owner } => {
                self.set_state(Some(OwnedAccountClosureState {
                    id: *id,
                    owner: *owner,
                    closed_account_count: 0,
                    rejected_account_count: 0,
                    status: OwnedAccountClosureStatus::Requested,
                }));
            }
            OwnedAccountClosureEventPayload::PageLoaded { .. } => {
                self.state_required_mut()?.status = OwnedAccountClosureStatus::InProgress;
            }
            OwnedAccountClosureEventPayload::PageLoadRejected { .. } => {}
            OwnedAccountClosureEventPayload::AccountCloseRecorded { .. } => {
                let state = self.state_required_mut()?;
                state.closed_account_count = state.closed_account_count.saturating_add(1);
            }
            OwnedAccountClosureEventPayload::AccountCloseRecordRejected { .. } => {}
            OwnedAccountClosureEventPayload::AccountCloseRejectionRecorded { .. } => {
                let state = self.state_required_mut()?;
                state.rejected_account_count = state.rejected_account_count.saturating_add(1);
            }
            OwnedAccountClosureEventPayload::AccountCloseRejectionRecordRejected { .. } => {}
            OwnedAccountClosureEventPayload::Completed { .. } => {
                self.state_required_mut()?.status = OwnedAccountClosureStatus::Completed;
            }
            OwnedAccountClosureEventPayload::CompleteRejected { .. } => {}
            OwnedAccountClosureEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = OwnedAccountClosureStatus::Failed;
            }
            OwnedAccountClosureEventPayload::FailRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};
    use banking_iam_domain::UserId;

    use crate::account::{AccountCloseRejectionReason, AccountId, AccountOwner};

    use super::{
        OwnedAccountClosure, OwnedAccountClosureCompleteRejectionReason,
        OwnedAccountClosureCompleteResult, OwnedAccountClosureEventPayload,
        OwnedAccountClosureRecordResult,
    };

    fn user_owner() -> AccountOwner {
        AccountOwner::User(UserId::new())
    }

    #[test]
    fn complete_rejects_before_any_page_is_loaded() {
        let mut closure = OwnedAccountClosure::default();
        closure
            .request(user_owner())
            .expect("request should succeed");

        let result = closure
            .complete()
            .expect("complete should emit a rejection event");

        assert!(matches!(
            result,
            OwnedAccountClosureCompleteResult::Rejected {
                reason: OwnedAccountClosureCompleteRejectionReason::NotInProgress
            }
        ));
        assert_eq!(
            closure.uncommitted_events()[1].payload().name(),
            OwnedAccountClosureEventPayload::COMPLETE_REJECTED
        );
    }

    #[test]
    fn complete_succeeds_after_an_empty_page_is_loaded() {
        let mut closure = OwnedAccountClosure::default();
        closure
            .request(user_owner())
            .expect("request should succeed");
        closure
            .load_page(Vec::new(), None)
            .expect("page load should succeed");

        let result = closure.complete().expect("complete should succeed");

        assert!(matches!(
            result,
            OwnedAccountClosureCompleteResult::Completed
        ));
        assert_eq!(
            closure.uncommitted_events()[2].payload().name(),
            OwnedAccountClosureEventPayload::COMPLETED
        );
    }

    #[test]
    fn complete_rejects_when_any_account_close_was_rejected() {
        let account_id = AccountId::new();
        let mut closure = OwnedAccountClosure::default();
        closure
            .request(user_owner())
            .expect("request should succeed");
        closure
            .load_page(vec![account_id], None)
            .expect("page load should succeed");
        let record_result = closure
            .record_account_close_rejection(
                account_id,
                AccountCloseRejectionReason::BalanceRemaining,
            )
            .expect("close rejection record should succeed");
        assert!(matches!(
            record_result,
            OwnedAccountClosureRecordResult::Recorded
        ));

        let result = closure
            .complete()
            .expect("complete should emit a rejection event");

        assert!(matches!(
            result,
            OwnedAccountClosureCompleteResult::Rejected {
                reason: OwnedAccountClosureCompleteRejectionReason::AccountCloseRejected
            }
        ));
        assert_eq!(
            closure.uncommitted_events()[3].payload().name(),
            OwnedAccountClosureEventPayload::COMPLETE_REJECTED
        );
    }
}
