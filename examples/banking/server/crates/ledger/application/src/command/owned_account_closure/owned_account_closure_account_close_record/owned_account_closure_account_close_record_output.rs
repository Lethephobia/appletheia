use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureRecordRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after recording an account close result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureAccountCloseRecordOutput {
    Recorded,
    Rejected {
        reason: OwnedAccountClosureRecordRejectionReason,
    },
}

impl CommandOutput for OwnedAccountClosureAccountCloseRecordOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
