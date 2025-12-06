use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventPayloadOwnedError {
    #[error("payload must not be null")]
    NullPayload,
    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),
}
