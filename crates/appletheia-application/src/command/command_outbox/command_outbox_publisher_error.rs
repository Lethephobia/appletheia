use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandOutboxPublisherError {
    #[error("command outbox publisher json error: {0}")]
    Json(#[from] serde_json::Error),
}
