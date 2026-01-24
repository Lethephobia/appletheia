use thiserror::Error;

use crate::outbox::command::SerializedCommandError;

#[derive(Debug, Error)]
pub enum SagaAppendCommandError {
    #[error("correlation id mismatch between saga instance and source event")]
    CorrelationIdMismatch,

    #[error("failed to serialize command: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid serialized command: {0}")]
    SerializedCommand(#[from] SerializedCommandError),
}
