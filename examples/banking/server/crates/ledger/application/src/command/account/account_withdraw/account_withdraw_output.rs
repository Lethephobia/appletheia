use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use serde::{Deserialize, Serialize};

/// Returned after an account withdraw request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountWithdrawOutput {
    Withdrawn,
}

impl CommandOutput for AccountWithdrawOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
