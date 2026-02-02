use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdempotencyOutputError {
    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),
}
