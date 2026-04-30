use crate::outbox::command::CommandEnvelopeError;
use thiserror::Error;

use crate::outbox::command::SerializedCommandError;

#[derive(Debug, Error)]
pub enum SagaInstanceError {
    #[error("correlation id mismatch between saga instance and source event")]
    CorrelationIdMismatch,

    #[error("no state")]
    NoState,

    #[error("failed to serialize command: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid serialized command: {0}")]
    SerializedCommand(#[from] SerializedCommandError),

    #[error("failed to build command envelope: {0}")]
    CommandEnvelope(#[from] CommandEnvelopeError),
}
