use appletheia::command;
use banking_ledger_domain::OrganizationLedgerRoleAssignmentId;
use serde::{Deserialize, Serialize};

#[command(name = "organization_ledger_role_assignment_unassign")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationLedgerRoleAssignmentUnassignCommand {
    pub organization_ledger_role_assignment_id: OrganizationLedgerRoleAssignmentId,
}
