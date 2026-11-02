use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::team_membership_cleanup::TeamMembershipCleanupId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipCleanupStartOutput {
    pub team_membership_cleanup_id: TeamMembershipCleanupId,
}

impl CommandOutput for TeamMembershipCleanupStartOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
