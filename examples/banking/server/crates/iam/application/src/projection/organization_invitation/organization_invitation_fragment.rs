use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationId, OrganizationInvitationExpiresAt, OrganizationInvitationId,
    OrganizationInvitationIssuer, OrganizationInvitationStatus, OrganizationRoles, UserId,
};
use serde::{Deserialize, Serialize};

/// Normalized organization invitation fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationInvitationFragment {
    pub invitation_id: OrganizationInvitationId,
    pub organization_id: OrganizationId,
    pub invitee_user_id: UserId,
    pub roles: OrganizationRoles,
    pub issuer: OrganizationInvitationIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for OrganizationInvitationFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
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
