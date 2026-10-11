use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::OrganizationLedgerRoleAssignmentId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationLedgerRoleAssignmentAssignOutput {
    pub organization_ledger_role_assignment_id: OrganizationLedgerRoleAssignmentId,
}

impl CommandOutput for OrganizationLedgerRoleAssignmentAssignOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
