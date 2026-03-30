use thiserror::Error;

use crate::command::FollowUpCommandError;

/// Represents errors returned while preparing a command failure reaction.
#[derive(Debug, Error)]
pub enum CommandFailureReactionError {
    #[error("failed to prepare a follow-up command")]
    FollowUpCommand(#[from] FollowUpCommandError),
}
