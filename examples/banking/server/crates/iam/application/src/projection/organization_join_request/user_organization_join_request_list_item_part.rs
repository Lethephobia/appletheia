use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName, ReadModelPartTree,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationJoinRequestId;

use crate::projection::InternalOrganizationSummaryPart;
use banking_iam_domain::OrganizationJoinRequestStatus;
use serde::{Deserialize, Serialize};

use super::OrganizationJoinRequestFragment;

/// One user organization join request list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserOrganizationJoinRequestListItemPart {
    pub join_request_id: OrganizationJoinRequestId,
    pub organization: InternalOrganizationSummaryPart,
    pub status: OrganizationJoinRequestStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<OrganizationJoinRequestFragment> for UserOrganizationJoinRequestListItemPart {
    fn from(fragment: OrganizationJoinRequestFragment) -> Self {
        Self {
            join_request_id: fragment.join_request_id,
            organization: fragment.organization.into(),
            status: fragment.status,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for UserOrganizationJoinRequestListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation, self.organization.observation]
    }
}

impl ReadModelPart for UserOrganizationJoinRequestListItemPart {
    const NAME: ReadModelPartName =
        ReadModelPartName::new("user_organization_join_request_list_item");

    type SourceFragment = OrganizationJoinRequestFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.join_request_id
    }

    fn parts(part: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::field::<InternalOrganizationSummaryPart>(
            "organization",
            part.map(|item| &item.organization),
        )]
    }
}
