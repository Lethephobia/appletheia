use banking_iam_domain::{
    OrganizationId, OrganizationInvitationExpiresAt, OrganizationInvitationId,
    OrganizationInvitationIssuer, OrganizationInvitationStatus, UserId,
};

/// Attributes required to upsert a normalized organization invitation projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInvitationProjectionUpsert {
    pub id: OrganizationInvitationId,
    pub organization_id: OrganizationId,
    pub invitee_id: UserId,
    pub issuer: OrganizationInvitationIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationStatus,
}
