use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializedSagaStepError {
    #[error("saga step must not be null")]
    Null,

    #[error("saga step json error: {0}")]
    Json(#[from] serde_json::Error),
}
