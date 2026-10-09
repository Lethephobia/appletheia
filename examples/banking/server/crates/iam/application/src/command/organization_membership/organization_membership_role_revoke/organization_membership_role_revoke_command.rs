use appletheia::command;
use banking_iam_domain::{OrganizationMembershipId, OrganizationRole};
use serde::{Deserialize, Serialize};

/// Revokes a role from an organization membership.
#[command(name = "organization_membership_role_revoke")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipRoleRevokeCommand {
    pub organization_membership_id: OrganizationMembershipId,
    pub role: OrganizationRole,
}
