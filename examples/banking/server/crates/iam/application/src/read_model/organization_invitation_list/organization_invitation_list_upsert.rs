use banking_iam_domain::{
    OrganizationId, OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles,
    UserId,
};

use super::{OrganizationInvitationListIssuer, OrganizationInvitationListItemStatus};

/// Describes an organization invitation list item upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInvitationListUpsert {
    pub invitation_id: OrganizationInvitationId,
    pub organization_id: OrganizationId,
    pub invitee_user_id: UserId,
    pub roles: OrganizationRoles,
    pub issuer: OrganizationInvitationListIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationListItemStatus,
}
