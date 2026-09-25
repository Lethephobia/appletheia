use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::transfer::TransferCompleteRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after completing a transfer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TransferCompleteOutput {
    Completed,
    Rejected {
        reason: TransferCompleteRejectionReason,
    },
}

impl CommandOutput for TransferCompleteOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
