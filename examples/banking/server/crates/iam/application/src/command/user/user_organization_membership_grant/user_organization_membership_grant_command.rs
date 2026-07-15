use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};
use serde::{Deserialize, Serialize};

/// Grants an organization membership to a user.
#[command(name = "user_organization_membership_grant")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserOrganizationMembershipGrantCommand {
    pub user_id: UserId,
    pub organization_id: OrganizationId,
    pub roles: OrganizationRoles,
}
