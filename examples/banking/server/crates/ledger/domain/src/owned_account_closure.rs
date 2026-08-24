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
mod owned_account_closure_request;
mod owned_account_closure_request_result;
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
pub use owned_account_closure_request::OwnedAccountClosureRequest;
pub use owned_account_closure_request_result::OwnedAccountClosureRequestResult;
pub use owned_account_closure_state::OwnedAccountClosureState;
pub use owned_account_closure_state_error::OwnedAccountClosureStateError;
pub use owned_account_closure_status::OwnedAccountClosureStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::{AccountCloseRejectionReason, AccountId, AccountOwner};

/// Represents the `OwnedAccountClosure` process aggregate.
#[aggregate(type = "owned_account_closure", error = OwnedAccountClosureError)]
pub struct OwnedAccountClosure {
    core: AggregateCore<
        OwnedAccountClosureId,
        OwnedAccountClosureState,
        OwnedAccountClosureEventPayload,
    >,
}

impl OwnedAccountClosure {
    /// Returns the owner whose accounts are being closed.
    pub fn owner(&self) -> Result<AccountOwner, OwnedAccountClosureError> {
        Ok(self.state_required()?.owner)
    }

    /// Returns the current closure status.
    pub fn status(&self) -> Result<OwnedAccountClosureStatus, OwnedAccountClosureError> {
        Ok(self.state_required()?.status)
    }

    /// Starts a workflow that closes every account owned by the owner.
    pub fn request(
        &mut self,
        request: OwnedAccountClosureRequest,
    ) -> Result<OwnedAccountClosureRequestResult, OwnedAccountClosureError> {
        if self.state().is_some() {
            return Err(OwnedAccountClosureError::AlreadyRequested);
        }

        let owner = request.into_owner();
        self.append_event(OwnedAccountClosureEventPayload::Requested { owner })?;

        Ok(OwnedAccountClosureRequestResult::Requested)
    }

    /// Records one page of accounts owned by the workflow owner.
    pub fn load_page(
        &mut self,
        account_ids: Vec<AccountId>,
        next_cursor: Option<AccountId>,
    ) -> Result<OwnedAccountClosurePageLoadResult, OwnedAccountClosureError> {
        if self.state_required()?.status.is_terminal() {
            let reason = OwnedAccountClosurePageLoadRejectionReason::AlreadyTerminal;
            self.reject_load_page(reason)?;
            return Ok(OwnedAccountClosurePageLoadResult::Rejected { reason });
        }

        self.append_event(OwnedAccountClosureEventPayload::PageLoaded {
            account_ids,
            next_cursor,
        })?;
        Ok(OwnedAccountClosurePageLoadResult::Loaded)
    }

    /// Rejects loading another owned account page.
    pub fn reject_load_page(
        &mut self,
        reason: OwnedAccountClosurePageLoadRejectionReason,
    ) -> Result<(), OwnedAccountClosureError> {
        self.append_event(OwnedAccountClosureEventPayload::PageLoadRejected { reason })?;
        Ok(())
    }

    /// Records a successful account close result.
    pub fn record_account_close(
        &mut self,
        account_id: AccountId,
    ) -> Result<OwnedAccountClosureRecordResult, OwnedAccountClosureError> {
        let state = self.state_required()?;
        if state.status.is_terminal() {
            let reason = OwnedAccountClosureRecordRejectionReason::AlreadyTerminal;
            self.reject_record_account_close(account_id, reason)?;
            return Ok(OwnedAccountClosureRecordResult::Rejected { reason });
        }

        self.append_event(OwnedAccountClosureEventPayload::AccountCloseRecorded { account_id })?;
        Ok(OwnedAccountClosureRecordResult::Recorded)
    }

    /// Rejects recording a successful account close result.
    pub fn reject_record_account_close(
        &mut self,
        account_id: AccountId,
        reason: OwnedAccountClosureRecordRejectionReason,
    ) -> Result<(), OwnedAccountClosureError> {
        self.append_event(
            OwnedAccountClosureEventPayload::AccountCloseRecordRejected { account_id, reason },
        )?;
        Ok(())
    }

    /// Records a rejected account close result.
    pub fn record_account_close_rejection(
        &mut self,
        account_id: AccountId,
        reason: AccountCloseRejectionReason,
    ) -> Result<OwnedAccountClosureRecordResult, OwnedAccountClosureError> {
        let state = self.state_required()?;
        if state.status.is_terminal() {
            let reason = OwnedAccountClosureRecordRejectionReason::AlreadyTerminal;
            self.reject_record_account_close_rejection(account_id, reason)?;
            return Ok(OwnedAccountClosureRecordResult::Rejected { reason });
        }

        self.append_event(
            OwnedAccountClosureEventPayload::AccountCloseRejectionRecorded { account_id, reason },
        )?;
        Ok(OwnedAccountClosureRecordResult::Recorded)
    }

    /// Rejects recording a rejected account close result.
    pub fn reject_record_account_close_rejection(
        &mut self,
        account_id: AccountId,
        reason: OwnedAccountClosureRecordRejectionReason,
    ) -> Result<(), OwnedAccountClosureError> {
        self.append_event(
            OwnedAccountClosureEventPayload::AccountCloseRejectionRecordRejected {
                account_id,
                reason,
            },
        )?;
        Ok(())
    }

    /// Marks the workflow completed.
    pub fn complete(
        &mut self,
    ) -> Result<OwnedAccountClosureCompleteResult, OwnedAccountClosureError> {
        let state = self.state_required()?;
        match state.status {
            OwnedAccountClosureStatus::Requested => {
                let reason = OwnedAccountClosureCompleteRejectionReason::NotInProgress;
                self.reject_complete(reason)?;
                return Ok(OwnedAccountClosureCompleteResult::Rejected { reason });
            }
            OwnedAccountClosureStatus::InProgress => {}
            OwnedAccountClosureStatus::Completed => {
                let reason = OwnedAccountClosureCompleteRejectionReason::AlreadyCompleted;
                self.reject_complete(reason)?;
                return Ok(OwnedAccountClosureCompleteResult::Rejected { reason });
            }
            OwnedAccountClosureStatus::Failed => {
                let reason = OwnedAccountClosureCompleteRejectionReason::AlreadyFailed;
                self.reject_complete(reason)?;
                return Ok(OwnedAccountClosureCompleteResult::Rejected { reason });
            }
        }

        let state = self.state_required()?;
        if state.rejected_account_count() > 0 {
            let reason = OwnedAccountClosureCompleteRejectionReason::AccountCloseRejected;
            self.reject_complete(reason)?;
            return Ok(OwnedAccountClosureCompleteResult::Rejected { reason });
        }

        let closed_account_count = state.closed_account_count();
        self.append_event(OwnedAccountClosureEventPayload::Completed {
            closed_account_count,
        })?;
        Ok(OwnedAccountClosureCompleteResult::Completed)
    }

    /// Rejects completing the closure workflow.
    pub fn reject_complete(
        &mut self,
        reason: OwnedAccountClosureCompleteRejectionReason,
    ) -> Result<(), OwnedAccountClosureError> {
        self.append_event(OwnedAccountClosureEventPayload::CompleteRejected { reason })?;
        Ok(())
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
                self.reject_fail(reason)?;
                return Ok(OwnedAccountClosureFailResult::Rejected { reason });
            }
            OwnedAccountClosureStatus::Failed => {
                let reason = OwnedAccountClosureFailRejectionReason::AlreadyFailed;
                self.reject_fail(reason)?;
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

    /// Rejects failing the closure workflow.
    pub fn reject_fail(
        &mut self,
        reason: OwnedAccountClosureFailRejectionReason,
    ) -> Result<(), OwnedAccountClosureError> {
        self.append_event(OwnedAccountClosureEventPayload::FailRejected { reason })?;
        Ok(())
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
            OwnedAccountClosureEventPayload::Requested { owner } => {
                self.set_state(Some(OwnedAccountClosureState {
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
        OwnedAccountClosureRecordResult, OwnedAccountClosureRequest, OwnedAccountClosureStatus,
    };

    fn user_owner() -> AccountOwner {
        AccountOwner::User(UserId::new())
    }

    #[test]
    fn complete_rejects_before_any_page_is_loaded() {
        let mut closure = OwnedAccountClosure::new();
        closure
            .request(OwnedAccountClosureRequest {
                owner: user_owner(),
            })
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
        let mut closure = OwnedAccountClosure::new();
        closure
            .request(OwnedAccountClosureRequest {
                owner: user_owner(),
            })
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
            closure.status().expect("closure state should exist"),
            OwnedAccountClosureStatus::Completed
        );
        assert_eq!(
            closure.uncommitted_events()[2].payload().name(),
            OwnedAccountClosureEventPayload::COMPLETED
        );
    }

    #[test]
    fn complete_rejects_when_any_account_close_was_rejected() {
        let account_id = AccountId::new();
        let mut closure = OwnedAccountClosure::new();
        closure
            .request(OwnedAccountClosureRequest {
                owner: user_owner(),
            })
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
