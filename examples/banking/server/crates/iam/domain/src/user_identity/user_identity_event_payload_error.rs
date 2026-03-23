use thiserror::Error;

/// Describes why a user-identity event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum UserIdentityEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
