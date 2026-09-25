use thiserror::Error;

/// Describes why an organization membership event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum OrganizationMembershipEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
