use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelAttributeValueError {
    #[error("attribute value serialization failed")]
    Json(#[from] serde_json::Error),
}
