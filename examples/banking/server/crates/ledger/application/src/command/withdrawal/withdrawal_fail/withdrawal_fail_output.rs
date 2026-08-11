use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::withdrawal::WithdrawalFailRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a withdrawal failure request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalFailOutput {
    Failed,
    Rejected {
        reason: WithdrawalFailRejectionReason,
    },
}

impl CommandOutput for WithdrawalFailOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
