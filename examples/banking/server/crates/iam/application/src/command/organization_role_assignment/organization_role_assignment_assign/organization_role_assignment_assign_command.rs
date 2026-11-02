use appletheia::command;
use banking_iam_domain::OrganizationId;
use banking_iam_domain::{OrganizationRole, OrganizationRoleAssignee};
use serde::{Deserialize, Serialize};

#[command(name = "organization_role_assignment_assign")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationRoleAssignmentAssignCommand {
    pub organization_id: OrganizationId,
    pub assignee: OrganizationRoleAssignee,
    pub role: OrganizationRole,
}
