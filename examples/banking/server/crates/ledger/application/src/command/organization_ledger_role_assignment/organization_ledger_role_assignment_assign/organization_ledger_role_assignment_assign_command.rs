use appletheia::command;
use banking_iam_domain::OrganizationId;
use banking_ledger_domain::{OrganizationLedgerRole, OrganizationLedgerRoleAssignee};
use serde::{Deserialize, Serialize};

#[command(name = "organization_ledger_role_assignment_assign")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationLedgerRoleAssignmentAssignCommand {
    pub organization_id: OrganizationId,
    pub assignee: OrganizationLedgerRoleAssignee,
    pub role: OrganizationLedgerRole,
}
