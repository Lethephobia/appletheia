use thiserror::Error;

use crate::outbox::command::SerializedCommandError;

/// Represents errors returned while preparing a follow-up command.
#[derive(Debug, Error)]
pub enum FollowUpCommandError {
    #[error("follow-up command payload could not be serialized")]
    Json(#[from] serde_json::Error),

    #[error("follow-up command payload is invalid")]
    SerializedCommand(#[from] SerializedCommandError),
}
