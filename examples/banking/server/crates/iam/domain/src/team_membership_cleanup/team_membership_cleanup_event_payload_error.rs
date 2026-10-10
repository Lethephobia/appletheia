use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipCleanupEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
