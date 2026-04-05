mod organization_join_request_error;
mod organization_join_request_event_payload;
mod organization_join_request_event_payload_error;
mod organization_join_request_id;
mod organization_join_request_state;
mod organization_join_request_state_error;
mod organization_join_request_status;

pub use organization_join_request_error::OrganizationJoinRequestError;
pub use organization_join_request_event_payload::OrganizationJoinRequestEventPayload;
pub use organization_join_request_event_payload_error::OrganizationJoinRequestEventPayloadError;
pub use organization_join_request_id::OrganizationJoinRequestId;
pub use organization_join_request_state::OrganizationJoinRequestState;
pub use organization_join_request_state_error::OrganizationJoinRequestStateError;
pub use organization_join_request_status::OrganizationJoinRequestStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::{OrganizationId, UserId};

/// Represents the `OrganizationJoinRequest` aggregate root.
#[aggregate(type = "organization_join_request", error = OrganizationJoinRequestError)]
pub struct OrganizationJoinRequest {
    core: AggregateCore<OrganizationJoinRequestState, OrganizationJoinRequestEventPayload>,
}

impl OrganizationJoinRequest {
    /// Returns the organization that received the join request.
    pub fn organization_id(&self) -> Result<&OrganizationId, OrganizationJoinRequestError> {
        Ok(&self.state_required()?.organization_id)
    }

    /// Returns the requesting user.
    pub fn requester_id(&self) -> Result<&UserId, OrganizationJoinRequestError> {
        Ok(&self.state_required()?.requester_id)
    }

    /// Returns the current join request status.
    pub fn status(&self) -> Result<OrganizationJoinRequestStatus, OrganizationJoinRequestError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the join request is pending.
    pub fn is_pending(&self) -> Result<bool, OrganizationJoinRequestError> {
        Ok(self.state_required()?.status.is_pending())
    }

    /// Returns whether the join request is approved.
    pub fn is_approved(&self) -> Result<bool, OrganizationJoinRequestError> {
        Ok(self.state_required()?.status.is_approved())
    }

    /// Returns whether the join request is rejected.
    pub fn is_rejected(&self) -> Result<bool, OrganizationJoinRequestError> {
        Ok(self.state_required()?.status.is_rejected())
    }

    /// Returns whether the join request is canceled.
    pub fn is_canceled(&self) -> Result<bool, OrganizationJoinRequestError> {
        Ok(self.state_required()?.status.is_canceled())
    }

    /// Requests to join an organization.
    pub fn request(
        &mut self,
        organization_id: OrganizationId,
        requester_id: UserId,
    ) -> Result<(), OrganizationJoinRequestError> {
        if self.state().is_some() {
            return Err(OrganizationJoinRequestError::AlreadyRequested);
        }

        self.append_event(OrganizationJoinRequestEventPayload::Requested {
            id: OrganizationJoinRequestId::new(),
            organization_id,
            requester_id,
        })
    }

    /// Approves the join request.
    pub fn approve(&mut self) -> Result<(), OrganizationJoinRequestError> {
        self.ensure_pending()?;

        let state = self.state_required()?;
        self.append_event(OrganizationJoinRequestEventPayload::Approved {
            organization_id: state.organization_id,
            requester_id: state.requester_id,
        })
    }

    /// Rejects the join request.
    pub fn reject(&mut self) -> Result<(), OrganizationJoinRequestError> {
        self.ensure_pending()?;

        let state = self.state_required()?;
        self.append_event(OrganizationJoinRequestEventPayload::Rejected {
            organization_id: state.organization_id,
            requester_id: state.requester_id,
        })
    }

    /// Cancels the join request.
    pub fn cancel(&mut self) -> Result<(), OrganizationJoinRequestError> {
        self.ensure_pending()?;

        let state = self.state_required()?;
        self.append_event(OrganizationJoinRequestEventPayload::Canceled {
            organization_id: state.organization_id,
            requester_id: state.requester_id,
        })
    }

    fn ensure_pending(&self) -> Result<(), OrganizationJoinRequestError> {
        if !self.state_required()?.status.is_pending() {
            return Err(OrganizationJoinRequestError::NotPending);
        }

        Ok(())
    }
}

impl AggregateApply<OrganizationJoinRequestEventPayload, OrganizationJoinRequestError>
    for OrganizationJoinRequest
{
    fn apply(
        &mut self,
        payload: &OrganizationJoinRequestEventPayload,
    ) -> Result<(), OrganizationJoinRequestError> {
        match payload {
            OrganizationJoinRequestEventPayload::Requested {
                id,
                organization_id,
                requester_id,
            } => {
                self.set_state(Some(OrganizationJoinRequestState::new(
                    *id,
                    *organization_id,
                    *requester_id,
                )));
            }
            OrganizationJoinRequestEventPayload::Approved { .. } => {
                self.state_required_mut()?.status = OrganizationJoinRequestStatus::Approved;
            }
            OrganizationJoinRequestEventPayload::Rejected { .. } => {
                self.state_required_mut()?.status = OrganizationJoinRequestStatus::Rejected;
            }
            OrganizationJoinRequestEventPayload::Canceled { .. } => {
                self.state_required_mut()?.status = OrganizationJoinRequestStatus::Canceled;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};

    use super::{
        OrganizationJoinRequest, OrganizationJoinRequestEventPayload, OrganizationJoinRequestStatus,
    };
    use crate::{OrganizationId, UserId};

    fn organization_id() -> OrganizationId {
        OrganizationId::new()
    }

    fn requester_id() -> UserId {
        UserId::new()
    }

    #[test]
    fn request_initializes_state_and_records_event() {
        let organization_id = organization_id();
        let requester_id = requester_id();
        let mut join_request = OrganizationJoinRequest::default();

        join_request
            .request(organization_id, requester_id)
            .expect("request should succeed");

        let aggregate_id = join_request
            .aggregate_id()
            .expect("aggregate id should exist");
        assert!(!aggregate_id.value().is_nil());
        assert_eq!(
            join_request
                .organization_id()
                .expect("organization id should exist"),
            &organization_id
        );
        assert_eq!(
            join_request
                .requester_id()
                .expect("requester id should exist"),
            &requester_id
        );
        assert_eq!(
            join_request.status().expect("status should exist"),
            OrganizationJoinRequestStatus::Pending
        );
        assert_eq!(join_request.uncommitted_events().len(), 1);
        assert_eq!(
            join_request.uncommitted_events()[0].payload().name(),
            OrganizationJoinRequestEventPayload::REQUESTED
        );
    }

    #[test]
    fn approving_request_updates_status_and_records_event() {
        let organization_id = organization_id();
        let requester_id = requester_id();
        let mut join_request = OrganizationJoinRequest::default();
        join_request
            .request(organization_id, requester_id)
            .expect("request should succeed");

        join_request.approve().expect("approve should succeed");

        assert_eq!(
            join_request.status().expect("status should exist"),
            OrganizationJoinRequestStatus::Approved
        );
        assert_eq!(join_request.uncommitted_events().len(), 2);
        assert_eq!(
            join_request.uncommitted_events()[1].payload().name(),
            OrganizationJoinRequestEventPayload::APPROVED
        );
    }

    #[test]
    fn rejecting_request_updates_status_and_records_event() {
        let organization_id = organization_id();
        let requester_id = requester_id();
        let mut join_request = OrganizationJoinRequest::default();
        join_request
            .request(organization_id, requester_id)
            .expect("request should succeed");

        join_request.reject().expect("reject should succeed");

        assert_eq!(
            join_request.status().expect("status should exist"),
            OrganizationJoinRequestStatus::Rejected
        );
        assert_eq!(join_request.uncommitted_events().len(), 2);
        assert_eq!(
            join_request.uncommitted_events()[1].payload().name(),
            OrganizationJoinRequestEventPayload::REJECTED
        );
    }

    #[test]
    fn canceling_request_updates_status_and_records_event() {
        let organization_id = organization_id();
        let requester_id = requester_id();
        let mut join_request = OrganizationJoinRequest::default();
        join_request
            .request(organization_id, requester_id)
            .expect("request should succeed");

        join_request.cancel().expect("cancel should succeed");

        assert_eq!(
            join_request.status().expect("status should exist"),
            OrganizationJoinRequestStatus::Canceled
        );
        assert_eq!(join_request.uncommitted_events().len(), 2);
        assert_eq!(
            join_request.uncommitted_events()[1].payload().name(),
            OrganizationJoinRequestEventPayload::CANCELED
        );
    }
}
