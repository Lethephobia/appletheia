use thiserror::Error;

/// Describes why an organization invitation event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum OrganizationInvitationEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
