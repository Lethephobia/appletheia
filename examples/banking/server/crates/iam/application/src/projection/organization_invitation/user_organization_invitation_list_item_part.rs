use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName, ReadModelPartTree,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles,
};

use crate::projection::InternalOrganizationSummaryPart;
use banking_iam_domain::{OrganizationInvitationIssuer, OrganizationInvitationStatus};
use serde::{Deserialize, Serialize};

use super::OrganizationInvitationFragment;

/// One user organization invitation list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserOrganizationInvitationListItemPart {
    pub invitation_id: OrganizationInvitationId,
    pub organization: InternalOrganizationSummaryPart,
    pub roles: OrganizationRoles,
    pub issuer: OrganizationInvitationIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<OrganizationInvitationFragment> for UserOrganizationInvitationListItemPart {
    fn from(fragment: OrganizationInvitationFragment) -> Self {
        Self {
            invitation_id: fragment.invitation_id,
            organization: fragment.organization.into(),
            roles: fragment.roles,
            issuer: fragment.issuer,
            expires_at: fragment.expires_at,
            status: fragment.status,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for UserOrganizationInvitationListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation, self.organization.observation]
    }
}

impl ReadModelPart for UserOrganizationInvitationListItemPart {
    const NAME: ReadModelPartName =
        ReadModelPartName::new("user_organization_invitation_list_item");

    type SourceFragment = OrganizationInvitationFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.invitation_id
    }

    fn parts(part: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::field::<InternalOrganizationSummaryPart>(
            "organization",
            part.map(|item| &item.organization),
        )]
    }
}
