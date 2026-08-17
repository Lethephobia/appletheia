use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationInvitationIssuer,
    OrganizationInvitationStatus, OrganizationRoles,
};
use serde::{Deserialize, Serialize};

use super::{OrganizationFragment, UserFragment};

/// Complete organization invitation fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationInvitationFragment {
    pub invitation_id: OrganizationInvitationId,
    pub organization: OrganizationFragment,
    pub invitee: UserFragment,
    pub roles: OrganizationRoles,
    pub issuer: OrganizationInvitationIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for OrganizationInvitationFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.organization
            .observations()
            .into_iter()
            .chain(self.invitee.observations())
            .chain([self.observation])
            .collect()
    }
}

impl ReadModelFragment for OrganizationInvitationFragment {
    const NAME: ReadModelFragmentName =
        ReadModelFragmentName::new("organization_invitation_fragment");

    type Key = OrganizationInvitationId;

    fn key(&self) -> Self::Key {
        self.invitation_id
    }
}
