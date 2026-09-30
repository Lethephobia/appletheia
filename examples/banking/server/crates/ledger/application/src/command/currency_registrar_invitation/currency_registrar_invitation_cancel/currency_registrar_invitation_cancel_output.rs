use appletheia::application::command::{CommandOutput, CommandReplayOutput};

use serde::{Deserialize, Serialize};

/// The output returned after canceling an currency registrar invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarInvitationCancelOutput {}

impl CommandOutput for CurrencyRegistrarInvitationCancelOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
