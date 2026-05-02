use banking_iam_domain::{
    OrganizationId, OrganizationInvitationExpiresAt, OrganizationInvitationId,
    OrganizationInvitationIssuer, OrganizationInvitationStatus, UserId,
};

/// Represents a normalized organization invitation view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInvitationView {
    pub id: OrganizationInvitationId,
    pub organization_id: OrganizationId,
    pub invitee_id: UserId,
    pub issuer: OrganizationInvitationIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationStatus,
}
