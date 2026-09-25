use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenBindingEventPayloadError {
    #[error("token binding event payload serialization failed")]
    Serialization(#[from] serde_json::Error),
}
