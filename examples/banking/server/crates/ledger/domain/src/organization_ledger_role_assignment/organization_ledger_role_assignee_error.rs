use appletheia::domain::UniqueValueError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationLedgerRoleAssigneeError {
    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),
}
