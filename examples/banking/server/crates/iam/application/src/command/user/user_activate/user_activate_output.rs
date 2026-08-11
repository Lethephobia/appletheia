use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::user::UserStatusRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a user activation request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserActivateOutput {
    Activated,
    Rejected { reason: UserStatusRejectionReason },
}

impl CommandOutput for UserActivateOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
