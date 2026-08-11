use thiserror::Error;

use super::SerializedCommandError;

#[derive(Debug, Error)]
pub enum CommandEnvelopeError {
    #[error("command name mismatch: expected {expected}, got {actual}")]
    CommandNameMismatch { expected: String, actual: String },

    #[error("json deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid serialized command: {0}")]
    SerializedCommand(#[from] SerializedCommandError),
}
