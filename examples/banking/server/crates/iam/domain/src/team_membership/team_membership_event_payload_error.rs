use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
