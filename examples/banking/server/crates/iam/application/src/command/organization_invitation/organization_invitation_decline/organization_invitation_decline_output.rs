use appletheia::application::command::{CommandOutput, CommandReplayOutput};

use serde::{Deserialize, Serialize};

/// The output returned after declining an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationDeclineOutput {}

impl CommandOutput for OrganizationInvitationDeclineOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
