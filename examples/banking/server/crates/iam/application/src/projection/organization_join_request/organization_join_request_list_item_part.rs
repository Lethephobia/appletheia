use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName, ReadModelPartTree,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationJoinRequestId;

use crate::projection::InternalUserSummaryPart;
use banking_iam_domain::OrganizationJoinRequestStatus;
use serde::{Deserialize, Serialize};

use super::OrganizationJoinRequestFragment;

/// One organization join request list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationJoinRequestListItemPart {
    pub join_request_id: OrganizationJoinRequestId,
    pub requester: InternalUserSummaryPart,
    pub status: OrganizationJoinRequestStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<OrganizationJoinRequestFragment> for OrganizationJoinRequestListItemPart {
    fn from(fragment: OrganizationJoinRequestFragment) -> Self {
        Self {
            join_request_id: fragment.join_request_id,
            requester: fragment.requester.into(),
            status: fragment.status,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for OrganizationJoinRequestListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation, self.requester.observation]
    }
}

impl ReadModelPart for OrganizationJoinRequestListItemPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("organization_join_request_list_item");

    type SourceFragment = OrganizationJoinRequestFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.join_request_id
    }

    fn parts(part: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::field::<InternalUserSummaryPart>(
            "requester",
            part.map(|item| &item.requester),
        )]
    }
}
