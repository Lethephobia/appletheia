use thiserror::Error;

/// Describes why an owned account closure event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum OwnedAccountClosureEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
