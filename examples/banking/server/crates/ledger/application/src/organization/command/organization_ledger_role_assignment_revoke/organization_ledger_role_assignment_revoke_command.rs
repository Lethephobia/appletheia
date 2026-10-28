use appletheia::command;
use banking_ledger_domain::OrganizationLedgerRoleAssignmentId;
use serde::{Deserialize, Serialize};

#[command(name = "organization_ledger_role_assignment_revoke")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationLedgerRoleAssignmentRevokeCommand {
    pub organization_ledger_role_assignment_id: OrganizationLedgerRoleAssignmentId,
}
