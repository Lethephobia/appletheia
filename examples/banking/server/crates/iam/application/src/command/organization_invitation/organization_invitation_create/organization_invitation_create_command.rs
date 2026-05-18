use appletheia::command;
use banking_iam_domain::{
    OrganizationId, OrganizationInvitationExpiresAt, OrganizationMembershipRoles, UserId,
};
use serde::{Deserialize, Serialize};

/// Issues an organization invitation.
#[command(name = "organization_invitation_issue")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationIssueCommand {
    pub organization_id: OrganizationId,
    pub invitee_id: UserId,
    pub roles: OrganizationMembershipRoles,
    pub expires_at: OrganizationInvitationExpiresAt,
}
