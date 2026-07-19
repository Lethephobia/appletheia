use banking_iam_domain::{
    OrganizationId, OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles,
    UserId,
};

use super::{UserOrganizationInvitationListIssuer, UserOrganizationInvitationListItemStatus};

/// Describes a user organization invitation list item upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserOrganizationInvitationListUpsert {
    pub invitation_id: OrganizationInvitationId,
    pub invitee_user_id: UserId,
    pub organization_id: OrganizationId,
    pub roles: OrganizationRoles,
    pub issuer: UserOrganizationInvitationListIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: UserOrganizationInvitationListItemStatus,
}
