use thiserror::Error;

/// Describes why an account event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum AccountEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
