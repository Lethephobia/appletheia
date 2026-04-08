use thiserror::Error;

/// Describes why an organization join request event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
