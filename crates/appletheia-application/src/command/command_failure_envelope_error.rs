use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandFailureEnvelopeError {
    #[error("command name mismatch: expected {expected}, got {actual}")]
    CommandNameMismatch { expected: String, actual: String },

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
