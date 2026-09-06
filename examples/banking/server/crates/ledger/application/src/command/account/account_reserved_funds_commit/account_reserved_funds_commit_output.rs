use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use serde::{Deserialize, Serialize};

/// Returned after committing reserved funds in an account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountReservedFundsCommitOutput {
    Committed,
}

impl CommandOutput for AccountReservedFundsCommitOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
