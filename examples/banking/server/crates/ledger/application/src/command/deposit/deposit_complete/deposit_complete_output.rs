use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::deposit::DepositCompleteRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after completing a deposit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DepositCompleteOutput {
    Completed,
    Rejected {
        reason: DepositCompleteRejectionReason,
    },
}

impl CommandOutput for DepositCompleteOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
