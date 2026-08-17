use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName, ReadModelPartTree,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles,
};

use crate::projection::InternalUserSummaryPart;
use banking_iam_domain::{OrganizationInvitationIssuer, OrganizationInvitationStatus};
use serde::{Deserialize, Serialize};

use super::OrganizationInvitationFragment;

/// One organization invitation list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationInvitationListItemPart {
    pub invitation_id: OrganizationInvitationId,
    pub invitee: InternalUserSummaryPart,
    pub roles: OrganizationRoles,
    pub issuer: OrganizationInvitationIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<OrganizationInvitationFragment> for OrganizationInvitationListItemPart {
    fn from(fragment: OrganizationInvitationFragment) -> Self {
        Self {
            invitation_id: fragment.invitation_id,
            invitee: fragment.invitee.into(),
            roles: fragment.roles,
            issuer: fragment.issuer,
            expires_at: fragment.expires_at,
            status: fragment.status,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for OrganizationInvitationListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation, self.invitee.observation]
    }
}

impl ReadModelPart for OrganizationInvitationListItemPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("organization_invitation_list_item");

    type SourceFragment = OrganizationInvitationFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        self.invitation_id
    }

    fn parts(part: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::field::<InternalUserSummaryPart>(
            "invitee",
            part.map(|item| &item.invitee),
        )]
    }
}
