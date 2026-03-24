use thiserror::Error;

/// Describes why a user event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum UserEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
