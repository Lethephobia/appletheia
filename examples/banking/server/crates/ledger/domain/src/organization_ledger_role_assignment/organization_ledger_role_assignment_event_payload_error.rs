use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationLedgerRoleAssignmentEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
