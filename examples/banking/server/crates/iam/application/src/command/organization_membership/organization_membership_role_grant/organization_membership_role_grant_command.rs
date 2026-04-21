use appletheia::command;
use banking_iam_domain::{OrganizationMembershipId, OrganizationRole};
use serde::{Deserialize, Serialize};

/// Grants a role to an organization membership.
#[command(name = "organization_membership_role_grant")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipRoleGrantCommand {
    pub organization_membership_id: OrganizationMembershipId,
    pub role: OrganizationRole,
}
