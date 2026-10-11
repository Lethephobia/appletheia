use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::TeamRoleAssignmentId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamRoleAssignmentAssignOutput {
    pub team_role_assignment_id: TeamRoleAssignmentId,
}

impl CommandOutput for TeamRoleAssignmentAssignOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
