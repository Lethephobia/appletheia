use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::user::UserPictureChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a user picture change request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserPictureChangeOutput {
    Changed,
    Rejected {
        reason: UserPictureChangeRejectionReason,
    },
}

impl CommandOutput for UserPictureChangeOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
