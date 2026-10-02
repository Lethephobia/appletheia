use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelAttributeError {
    #[error("attribute value serialization failed")]
    Json(#[from] serde_json::Error),
}
