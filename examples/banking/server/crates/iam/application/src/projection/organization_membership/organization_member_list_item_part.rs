use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName, ReadModelPartTree,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{OrganizationId, OrganizationRoles};
use serde::{Deserialize, Serialize};

use crate::projection::InternalUserSummaryPart;

use super::{OrganizationMembershipFragment, OrganizationMembershipFragmentKey};

/// One organization member list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMemberListItemPart {
    pub organization_id: OrganizationId,
    pub member: InternalUserSummaryPart,
    pub roles: OrganizationRoles,
    pub is_owner: bool,
    pub joined_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<OrganizationMembershipFragment> for OrganizationMemberListItemPart {
    fn from(fragment: OrganizationMembershipFragment) -> Self {
        let is_owner = fragment.user.id == fragment.organization.owner.id;

        Self {
            organization_id: fragment.organization.id,
            member: fragment.user.into(),
            roles: fragment.roles,
            is_owner,
            joined_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for OrganizationMemberListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation, self.member.observation]
    }
}

impl ReadModelPart for OrganizationMemberListItemPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("organization_member_list_item");

    type SourceFragment = OrganizationMembershipFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        OrganizationMembershipFragmentKey {
            user_id: self.member.user_id,
            organization_id: self.organization_id,
        }
    }

    fn parts(part: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::field::<InternalUserSummaryPart>(
            "member",
            part.map(|item| &item.member),
        )]
    }
}
