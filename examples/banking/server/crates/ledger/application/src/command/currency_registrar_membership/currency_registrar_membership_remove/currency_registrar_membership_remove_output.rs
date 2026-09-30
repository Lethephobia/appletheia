use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyRegistrarMembershipRemoveOutput {}

impl CommandOutput for CurrencyRegistrarMembershipRemoveOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
