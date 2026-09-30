use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_ledger_domain::withdrawal::WithdrawalId;
use serde::{Deserialize, Serialize};

/// Returned after a withdrawal request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalRequestOutput {
    pub withdrawal_id: WithdrawalId,
}

impl CommandOutput for WithdrawalRequestOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
