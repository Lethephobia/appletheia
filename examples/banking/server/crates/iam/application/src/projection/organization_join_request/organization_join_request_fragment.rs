use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationId, OrganizationJoinRequestId, OrganizationJoinRequestStatus, UserId,
};
use serde::{Deserialize, Serialize};

/// Normalized organization join request fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestFragment {
    pub join_request_id: OrganizationJoinRequestId,
    pub organization_id: OrganizationId,
    pub requester_user_id: UserId,
    pub status: OrganizationJoinRequestStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for OrganizationJoinRequestFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelFragment for OrganizationJoinRequestFragment {
    const NAME: ReadModelFragmentName =
        ReadModelFragmentName::new("organization_join_request_fragment");

    type Key = OrganizationJoinRequestId;

    fn key(&self) -> Self::Key {
        self.join_request_id
    }
}
