use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::transfer::TransferFailRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after failing a transfer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TransferFailOutput {
    Failed,
    Rejected { reason: TransferFailRejectionReason },
}

impl CommandOutput for TransferFailOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
