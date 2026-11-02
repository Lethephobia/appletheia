use appletheia::command;
use banking_iam_domain::OrganizationRoleAssignmentId;
use serde::{Deserialize, Serialize};

#[command(name = "organization_role_assignment_unassign")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationRoleAssignmentUnassignCommand {
    pub organization_role_assignment_id: OrganizationRoleAssignmentId,
}
