mod currency_registrar_join_request_approve_rejection_reason;
mod currency_registrar_join_request_approve_result;
mod currency_registrar_join_request_cancel_rejection_reason;
mod currency_registrar_join_request_cancel_result;
mod currency_registrar_join_request_error;
mod currency_registrar_join_request_event_payload;
mod currency_registrar_join_request_event_payload_error;
mod currency_registrar_join_request_id;
mod currency_registrar_join_request_reject_rejection_reason;
mod currency_registrar_join_request_reject_result;
mod currency_registrar_join_request_state;
mod currency_registrar_join_request_state_error;
mod currency_registrar_join_request_status;
mod currency_registrar_join_request_submission;
mod currency_registrar_join_request_submit_rejection_reason;
mod currency_registrar_join_request_submit_result;

pub use currency_registrar_join_request_approve_rejection_reason::CurrencyRegistrarJoinRequestApproveRejectionReason;
pub use currency_registrar_join_request_approve_result::CurrencyRegistrarJoinRequestApproveResult;
pub use currency_registrar_join_request_cancel_rejection_reason::CurrencyRegistrarJoinRequestCancelRejectionReason;
pub use currency_registrar_join_request_cancel_result::CurrencyRegistrarJoinRequestCancelResult;
pub use currency_registrar_join_request_error::CurrencyRegistrarJoinRequestError;
pub use currency_registrar_join_request_event_payload::CurrencyRegistrarJoinRequestEventPayload;
pub use currency_registrar_join_request_event_payload_error::CurrencyRegistrarJoinRequestEventPayloadError;
pub use currency_registrar_join_request_id::CurrencyRegistrarJoinRequestId;
pub use currency_registrar_join_request_reject_rejection_reason::CurrencyRegistrarJoinRequestRejectRejectionReason;
pub use currency_registrar_join_request_reject_result::CurrencyRegistrarJoinRequestRejectResult;
pub use currency_registrar_join_request_state::CurrencyRegistrarJoinRequestState;
pub use currency_registrar_join_request_state_error::CurrencyRegistrarJoinRequestStateError;
pub use currency_registrar_join_request_status::CurrencyRegistrarJoinRequestStatus;
pub use currency_registrar_join_request_submission::CurrencyRegistrarJoinRequestSubmission;
pub use currency_registrar_join_request_submit_rejection_reason::CurrencyRegistrarJoinRequestSubmitRejectionReason;
pub use currency_registrar_join_request_submit_result::CurrencyRegistrarJoinRequestSubmitResult;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

/// Represents the `CurrencyRegistrarJoinRequest` aggregate root.
#[aggregate(type = "currency_registrar_join_request", error = CurrencyRegistrarJoinRequestError)]
pub struct CurrencyRegistrarJoinRequest {
    core: AggregateCore<
        CurrencyRegistrarJoinRequestId,
        CurrencyRegistrarJoinRequestState,
        CurrencyRegistrarJoinRequestEventPayload,
    >,
}

impl CurrencyRegistrarJoinRequest {
    /// Returns the registrar that received the join request.
    pub fn currency_registrar_id(
        &self,
    ) -> Result<&CurrencyRegistrarId, CurrencyRegistrarJoinRequestError> {
        Ok(&self.state_required()?.currency_registrar_id)
    }

    /// Returns the requesting user.
    pub fn requester_id(&self) -> Result<&UserId, CurrencyRegistrarJoinRequestError> {
        Ok(&self.state_required()?.requester_id)
    }

    /// Returns the current join request status.
    pub fn status(
        &self,
    ) -> Result<CurrencyRegistrarJoinRequestStatus, CurrencyRegistrarJoinRequestError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the join request is pending.
    pub fn is_pending(&self) -> Result<bool, CurrencyRegistrarJoinRequestError> {
        Ok(self.state_required()?.status.is_pending())
    }

    /// Returns whether the join request is approved.
    pub fn is_approved(&self) -> Result<bool, CurrencyRegistrarJoinRequestError> {
        Ok(self.state_required()?.status.is_approved())
    }

    /// Returns whether the join request is rejected.
    pub fn is_rejected(&self) -> Result<bool, CurrencyRegistrarJoinRequestError> {
        Ok(self.state_required()?.status.is_rejected())
    }

    /// Returns whether the join request is canceled.
    pub fn is_canceled(&self) -> Result<bool, CurrencyRegistrarJoinRequestError> {
        Ok(self.state_required()?.status.is_canceled())
    }

    /// Submits a request to join an registrar.
    pub fn submit(
        &mut self,
        submission: CurrencyRegistrarJoinRequestSubmission,
    ) -> Result<CurrencyRegistrarJoinRequestSubmitResult, CurrencyRegistrarJoinRequestError> {
        if self.state().is_some() {
            return Err(CurrencyRegistrarJoinRequestError::AlreadySubmitted);
        }

        let (currency_registrar_id, requester_id) = submission.into_parts();
        self.append_event(CurrencyRegistrarJoinRequestEventPayload::Submitted {
            currency_registrar_id,
            requester_id,
        })?;
        Ok(CurrencyRegistrarJoinRequestSubmitResult::Submitted)
    }

    /// Rejects a join request submission attempt.
    pub fn reject_submit(
        &mut self,
        _submission: CurrencyRegistrarJoinRequestSubmission,
        reason: CurrencyRegistrarJoinRequestSubmitRejectionReason,
    ) -> Result<(), CurrencyRegistrarJoinRequestError> {
        Err(CurrencyRegistrarJoinRequestError::SubmitRejected(reason))
    }

    /// Approves the join request.
    pub fn approve(
        &mut self,
    ) -> Result<CurrencyRegistrarJoinRequestApproveResult, CurrencyRegistrarJoinRequestError> {
        if !self.state_required()?.status.is_pending() {
            let reason = CurrencyRegistrarJoinRequestApproveRejectionReason::NotPending;
            self.reject_approve(reason)?;
            return Ok(CurrencyRegistrarJoinRequestApproveResult::Rejected { reason });
        }
        let state = self.state_required()?;
        self.append_event(CurrencyRegistrarJoinRequestEventPayload::Approved {
            currency_registrar_id: state.currency_registrar_id,
            requester_id: state.requester_id,
        })?;
        Ok(CurrencyRegistrarJoinRequestApproveResult::Approved)
    }

    /// Rejects a join request approval attempt.
    pub fn reject_approve(
        &mut self,
        reason: CurrencyRegistrarJoinRequestApproveRejectionReason,
    ) -> Result<(), CurrencyRegistrarJoinRequestError> {
        Err(CurrencyRegistrarJoinRequestError::ApproveRejected(reason))
    }

    /// Rejects the join request.
    pub fn reject(
        &mut self,
    ) -> Result<CurrencyRegistrarJoinRequestRejectResult, CurrencyRegistrarJoinRequestError> {
        if !self.state_required()?.status.is_pending() {
            let reason = CurrencyRegistrarJoinRequestRejectRejectionReason::NotPending;
            self.reject_rejection(reason)?;
            return Ok(CurrencyRegistrarJoinRequestRejectResult::RejectionRejected { reason });
        }
        let state = self.state_required()?;
        self.append_event(CurrencyRegistrarJoinRequestEventPayload::Rejected {
            currency_registrar_id: state.currency_registrar_id,
            requester_id: state.requester_id,
        })?;
        Ok(CurrencyRegistrarJoinRequestRejectResult::Rejected)
    }

    /// Rejects a join request rejection attempt.
    pub fn reject_rejection(
        &mut self,
        reason: CurrencyRegistrarJoinRequestRejectRejectionReason,
    ) -> Result<(), CurrencyRegistrarJoinRequestError> {
        Err(CurrencyRegistrarJoinRequestError::RejectRejected(reason))
    }

    /// Cancels the join request.
    pub fn cancel(
        &mut self,
    ) -> Result<CurrencyRegistrarJoinRequestCancelResult, CurrencyRegistrarJoinRequestError> {
        if !self.state_required()?.status.is_pending() {
            let reason = CurrencyRegistrarJoinRequestCancelRejectionReason::NotPending;
            self.reject_cancel(reason)?;
            return Ok(CurrencyRegistrarJoinRequestCancelResult::Rejected { reason });
        }
        let state = self.state_required()?;
        self.append_event(CurrencyRegistrarJoinRequestEventPayload::Canceled {
            currency_registrar_id: state.currency_registrar_id,
            requester_id: state.requester_id,
        })?;
        Ok(CurrencyRegistrarJoinRequestCancelResult::Canceled)
    }

    /// Rejects a join request cancellation attempt.
    pub fn reject_cancel(
        &mut self,
        reason: CurrencyRegistrarJoinRequestCancelRejectionReason,
    ) -> Result<(), CurrencyRegistrarJoinRequestError> {
        Err(CurrencyRegistrarJoinRequestError::CancelRejected(reason))
    }
}

impl AggregateApply<CurrencyRegistrarJoinRequestEventPayload, CurrencyRegistrarJoinRequestError>
    for CurrencyRegistrarJoinRequest
{
    fn apply(
        &mut self,
        payload: &CurrencyRegistrarJoinRequestEventPayload,
    ) -> Result<(), CurrencyRegistrarJoinRequestError> {
        match payload {
            CurrencyRegistrarJoinRequestEventPayload::Submitted {
                currency_registrar_id,
                requester_id,
            } => {
                self.set_state(Some(CurrencyRegistrarJoinRequestState {
                    currency_registrar_id: *currency_registrar_id,
                    requester_id: *requester_id,
                    status: CurrencyRegistrarJoinRequestStatus::Pending,
                }));
            }
            CurrencyRegistrarJoinRequestEventPayload::Approved { .. } => {
                self.state_required_mut()?.status = CurrencyRegistrarJoinRequestStatus::Approved;
            }
            CurrencyRegistrarJoinRequestEventPayload::Rejected { .. } => {
                self.state_required_mut()?.status = CurrencyRegistrarJoinRequestStatus::Rejected;
            }
            CurrencyRegistrarJoinRequestEventPayload::Canceled { .. } => {
                self.state_required_mut()?.status = CurrencyRegistrarJoinRequestStatus::Canceled;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::Aggregate;
    use banking_iam_domain::UserId;

    use super::{
        CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestApproveResult,
        CurrencyRegistrarJoinRequestSubmission, CurrencyRegistrarJoinRequestSubmitResult,
    };
    use crate::currency_registrar::CurrencyRegistrarId;

    #[test]
    fn approved_request_is_terminal_and_repeated_approval_is_recorded() {
        let mut request = CurrencyRegistrarJoinRequest::new();
        assert_eq!(
            request
                .submit(CurrencyRegistrarJoinRequestSubmission {
                    currency_registrar_id: CurrencyRegistrarId::new(),
                    requester_id: UserId::new(),
                })
                .expect("request should be submitted"),
            CurrencyRegistrarJoinRequestSubmitResult::Submitted
        );
        assert_eq!(
            request.approve().expect("first approval should succeed"),
            CurrencyRegistrarJoinRequestApproveResult::Approved
        );
        request
            .approve()
            .expect_err("repeated approval should fail");
        assert_eq!(request.uncommitted_events().len(), 2);
    }
}
