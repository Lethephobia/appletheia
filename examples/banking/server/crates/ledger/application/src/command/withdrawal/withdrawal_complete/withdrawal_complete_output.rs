use appletheia::application::command::{CommandOutput, CommandReplayOutput};

use serde::{Deserialize, Serialize};

/// Returned after a withdrawal completion request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalCompleteOutput {}

impl CommandOutput for WithdrawalCompleteOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
