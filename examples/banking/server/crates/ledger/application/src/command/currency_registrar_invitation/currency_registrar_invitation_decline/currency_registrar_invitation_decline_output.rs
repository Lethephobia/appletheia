use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarInvitationDeclineOutput {}

impl CommandOutput for CurrencyRegistrarInvitationDeclineOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
