use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandEnvelopeCommandError {
    #[error("command name mismatch: expected {expected}, got {actual}")]
    CommandNameMismatch { expected: String, actual: String },

    #[error("json deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}
