use appletheia::domain::UniqueValueError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationRoleAssigneeError {
    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),
}
