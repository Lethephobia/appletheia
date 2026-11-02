use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::TeamMembershipId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipCreateOutput {
    pub team_membership_id: TeamMembershipId,
}

impl CommandOutput for TeamMembershipCreateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
