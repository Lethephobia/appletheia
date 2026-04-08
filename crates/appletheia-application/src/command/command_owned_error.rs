use thiserror::Error;

use crate::outbox::command::SerializedCommandError;

/// Represents errors returned while preparing an owned command.
#[derive(Debug, Error)]
pub enum CommandOwnedError {
    #[error("command payload could not be serialized")]
    Json(#[from] serde_json::Error),

    #[error("command payload is invalid")]
    SerializedCommand(#[from] SerializedCommandError),
}
