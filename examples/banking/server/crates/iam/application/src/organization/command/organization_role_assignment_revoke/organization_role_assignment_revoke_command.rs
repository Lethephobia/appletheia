use appletheia::command;
use banking_iam_domain::OrganizationRoleAssignmentId;
use serde::{Deserialize, Serialize};

#[command(name = "organization_role_assignment_revoke")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationRoleAssignmentRevokeCommand {
    pub organization_role_assignment_id: OrganizationRoleAssignmentId,
}
