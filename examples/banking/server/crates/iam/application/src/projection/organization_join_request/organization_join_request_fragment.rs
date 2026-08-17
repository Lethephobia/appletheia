use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{OrganizationJoinRequestId, OrganizationJoinRequestStatus};
use serde::{Deserialize, Serialize};

use super::{OrganizationFragment, UserFragment};

/// Complete organization join request fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestFragment {
    pub join_request_id: OrganizationJoinRequestId,
    pub organization: OrganizationFragment,
    pub requester: UserFragment,
    pub status: OrganizationJoinRequestStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for OrganizationJoinRequestFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.organization
            .observations()
            .into_iter()
            .chain(self.requester.observations())
            .chain([self.observation])
            .collect()
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
