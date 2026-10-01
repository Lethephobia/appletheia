use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles,
};

use super::{
    UserOrganizationInvitationListIssuer, UserOrganizationInvitationListItemStatus,
    UserOrganizationInvitationListOrganization,
};

/// One user organization invitation list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationInvitationListItem {
    pub invitation_id: OrganizationInvitationId,
    pub organization: UserOrganizationInvitationListOrganization,
    pub roles: OrganizationRoles,
    pub issuer: UserOrganizationInvitationListIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: UserOrganizationInvitationListItemStatus,
    pub created_at: EventOccurredAt,
}
