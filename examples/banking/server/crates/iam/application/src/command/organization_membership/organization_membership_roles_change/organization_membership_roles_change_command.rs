use appletheia::command;
use banking_iam_domain::{OrganizationMembershipId, OrganizationRoles};
use serde::{Deserialize, Serialize};

/// Changes the roles granted by an organization membership.
#[command(name = "organization_membership_roles_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipRolesChangeCommand {
    pub organization_membership_id: OrganizationMembershipId,
    pub roles: OrganizationRoles,
}
