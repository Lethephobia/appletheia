use thiserror::Error;

/// Describes why a user-role-assignment event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum UserRoleAssignmentEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
