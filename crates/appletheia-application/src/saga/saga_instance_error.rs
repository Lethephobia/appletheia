use thiserror::Error;

use crate::command::{CommandEnvelopeError, SerializedCommandError};
use crate::saga::SerializedSagaStepError;

#[derive(Debug, Error)]
pub enum SagaInstanceError {
    #[error("no state")]
    NoState,

    #[error("failed to serialize command: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid serialized command: {0}")]
    SerializedCommand(#[from] SerializedCommandError),

    #[error("failed to build command envelope: {0}")]
    CommandEnvelope(#[from] CommandEnvelopeError),

    #[error("failed to serialize saga step: {0}")]
    SerializedSagaStep(#[from] SerializedSagaStepError),
}
