use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles,
};

use super::{
    OrganizationInvitationListInvitee, OrganizationInvitationListIssuer,
    OrganizationInvitationListItemStatus,
};

/// One organization invitation list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationInvitationListItem {
    pub invitation_id: OrganizationInvitationId,
    pub invitee: OrganizationInvitationListInvitee,
    pub roles: OrganizationRoles,
    pub issuer: OrganizationInvitationListIssuer,
    pub expires_at: OrganizationInvitationExpiresAt,
    pub status: OrganizationInvitationListItemStatus,
    pub created_at: EventOccurredAt,
}
