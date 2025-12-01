use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutboxPublisherError {
    #[error("outbox publisher json error: {0}")]
    Json(#[from] serde_json::Error),
}
