use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationRoleAssignmentEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
