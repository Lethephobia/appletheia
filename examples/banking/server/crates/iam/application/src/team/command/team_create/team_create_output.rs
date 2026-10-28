use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::TeamId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamCreateOutput {
    pub team_id: TeamId,
}

impl CommandOutput for TeamCreateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
