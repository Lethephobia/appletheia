use banking_iam_domain::{
    OrganizationId, OrganizationInvitationExpiresAt, OrganizationInvitationId,
    OrganizationInvitationIssuer, OrganizationInvitationStatus, OrganizationRoles, UserId,
};

/// Values used to create or replace an organization invitation fragment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInvitationFragmentUpsert {
    pub invitation_id: OrganizationInvitationId,
    pub organization_id: OrganizationId,
    pub invitee_user_id: UserId,
    pub roles: OrganizationRoles,
    pub issuer: OrganizationInvitationIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationStatus,
}
