use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};
use serde::{Deserialize, Serialize};

/// Changes a user's roles in an organization.
#[command(name = "user_organization_membership_roles_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserOrganizationMembershipRolesChangeCommand {
    pub user_id: UserId,
    pub organization_id: OrganizationId,
    pub roles: OrganizationRoles,
}
