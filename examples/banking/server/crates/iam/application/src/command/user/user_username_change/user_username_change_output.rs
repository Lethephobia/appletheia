use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::user::UserUsernameChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a username change request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserUsernameChangeOutput {
    Changed,
    Rejected {
        reason: UserUsernameChangeRejectionReason,
    },
}

impl CommandOutput for UserUsernameChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
