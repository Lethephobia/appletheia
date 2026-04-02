use thiserror::Error;

/// Describes why an organization event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum OrganizationEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
