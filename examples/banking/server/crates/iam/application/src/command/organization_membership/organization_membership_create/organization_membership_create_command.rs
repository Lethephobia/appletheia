use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};
use serde::{Deserialize, Serialize};

/// Creates an organization membership for a user.
#[command(name = "organization_membership_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipCreateCommand {
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub roles: OrganizationRoles,
}
