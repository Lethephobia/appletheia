use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamRoleAssignmentEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
