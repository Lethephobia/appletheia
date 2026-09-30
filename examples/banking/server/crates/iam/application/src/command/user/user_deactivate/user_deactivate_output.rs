use appletheia::application::command::{CommandOutput, CommandReplayOutput};

use serde::{Deserialize, Serialize};

/// Returned after a user deactivation request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDeactivateOutput {}

impl CommandOutput for UserDeactivateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
