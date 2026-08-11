use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::deposit::DepositFailRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after failing a deposit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DepositFailOutput {
    Failed,
    Rejected { reason: DepositFailRejectionReason },
}

impl CommandOutput for DepositFailOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
