mod owned_account_closure_error;
mod owned_account_closure_event_payload;
mod owned_account_closure_event_payload_error;
mod owned_account_closure_failure_reason;
mod owned_account_closure_id;
mod owned_account_closure_request;
mod owned_account_closure_state;
mod owned_account_closure_state_error;
mod owned_account_closure_status;

pub use owned_account_closure_error::OwnedAccountClosureError;
pub use owned_account_closure_event_payload::OwnedAccountClosureEventPayload;
pub use owned_account_closure_event_payload_error::OwnedAccountClosureEventPayloadError;
pub use owned_account_closure_failure_reason::OwnedAccountClosureFailureReason;
pub use owned_account_closure_id::OwnedAccountClosureId;
pub use owned_account_closure_request::OwnedAccountClosureRequest;
pub use owned_account_closure_state::OwnedAccountClosureState;
pub use owned_account_closure_state_error::OwnedAccountClosureStateError;
pub use owned_account_closure_status::OwnedAccountClosureStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::{AccountId, AccountOwner};

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
    ) -> Result<(), OwnedAccountClosureError> {
        if self.state().is_some() {
            return Err(OwnedAccountClosureError::AlreadyRequested);
        }

        let owner = request.into_owner();
        self.append_event(OwnedAccountClosureEventPayload::Requested { owner })?;

        Ok(())
    }

    /// Records one page of accounts owned by the workflow owner.
    pub fn load_page(
        &mut self,
        account_ids: Vec<AccountId>,
        next_cursor: Option<AccountId>,
    ) -> Result<(), OwnedAccountClosureError> {
        if self.state_required()?.status.is_terminal() {
            return Err(OwnedAccountClosureError::AlreadyFinished);
        }

        self.append_event(OwnedAccountClosureEventPayload::PageLoaded {
            account_ids,
            next_cursor,
        })?;
        Ok(())
    }

    /// Records a successful account close result.
    pub fn record_account_close(
        &mut self,
        account_id: AccountId,
    ) -> Result<(), OwnedAccountClosureError> {
        let state = self.state_required()?;
        if state.status.is_terminal() {
            return Err(OwnedAccountClosureError::AlreadyFinished);
        }

        self.append_event(OwnedAccountClosureEventPayload::AccountCloseRecorded { account_id })?;
        Ok(())
    }

    /// Records a rejected account close result.
    pub fn record_account_close_rejection(
        &mut self,
        account_id: AccountId,
    ) -> Result<(), OwnedAccountClosureError> {
        let state = self.state_required()?;
        if state.status.is_terminal() {
            return Err(OwnedAccountClosureError::AlreadyFinished);
        }

        self.append_event(
            OwnedAccountClosureEventPayload::AccountCloseRejectionRecorded { account_id },
        )?;
        Ok(())
    }

    /// Marks the workflow completed.
    pub fn complete(&mut self) -> Result<(), OwnedAccountClosureError> {
        let state = self.state_required()?;
        match state.status {
            OwnedAccountClosureStatus::Requested => {
                return Err(OwnedAccountClosureError::NotInProgress);
            }
            OwnedAccountClosureStatus::InProgress => {}
            OwnedAccountClosureStatus::Completed => {
                return Err(OwnedAccountClosureError::AlreadyCompleted);
            }
            OwnedAccountClosureStatus::Failed => {
                return Err(OwnedAccountClosureError::AlreadyFailed);
            }
        }

        let state = self.state_required()?;
        if state.rejected_account_count() > 0 {
            return Err(OwnedAccountClosureError::AccountClosureFailed);
        }

        let closed_account_count = state.closed_account_count();
        self.append_event(OwnedAccountClosureEventPayload::Completed {
            closed_account_count,
        })?;
        Ok(())
    }

    /// Marks the workflow failed after all account close attempts were recorded.
    pub fn fail(
        &mut self,
        reason: OwnedAccountClosureFailureReason,
    ) -> Result<(), OwnedAccountClosureError> {
        match self.state_required()?.status {
            OwnedAccountClosureStatus::Requested | OwnedAccountClosureStatus::InProgress => {}
            OwnedAccountClosureStatus::Completed => {
                return Err(OwnedAccountClosureError::AlreadyCompleted);
            }
            OwnedAccountClosureStatus::Failed => {
                return Err(OwnedAccountClosureError::AlreadyFailed);
            }
        }

        let state = self.state_required()?;
        self.append_event(OwnedAccountClosureEventPayload::Failed {
            closed_account_count: state.closed_account_count(),
            rejected_account_count: state.rejected_account_count(),
            reason,
        })?;
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
            OwnedAccountClosureEventPayload::AccountCloseRecorded { .. } => {
                let state = self.state_required_mut()?;
                state.closed_account_count = state.closed_account_count.saturating_add(1);
            }
            OwnedAccountClosureEventPayload::AccountCloseRejectionRecorded { .. } => {
                let state = self.state_required_mut()?;
                state.rejected_account_count = state.rejected_account_count.saturating_add(1);
            }
            OwnedAccountClosureEventPayload::Completed { .. } => {
                self.state_required_mut()?.status = OwnedAccountClosureStatus::Completed;
            }
            OwnedAccountClosureEventPayload::Failed { .. } => {
                self.state_required_mut()?.status = OwnedAccountClosureStatus::Failed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};
    use banking_iam_domain::UserId;

    use crate::account::{AccountId, AccountOwner};

    use super::{
        OwnedAccountClosure, OwnedAccountClosureEventPayload, OwnedAccountClosureRequest,
        OwnedAccountClosureStatus,
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

        closure.complete().expect_err("complete should fail");
        assert_eq!(closure.uncommitted_events().len(), 1);
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

        closure.complete().expect("complete should succeed");

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
        closure
            .record_account_close_rejection(account_id)
            .expect("close rejection record should succeed");

        closure.complete().expect_err("complete should fail");
        assert_eq!(closure.uncommitted_events().len(), 3);
    }
}
