use thiserror::Error;

/// Describes why a team event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum TeamEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
