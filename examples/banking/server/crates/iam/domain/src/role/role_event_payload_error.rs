use thiserror::Error;

/// Describes why a role event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum RoleEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
