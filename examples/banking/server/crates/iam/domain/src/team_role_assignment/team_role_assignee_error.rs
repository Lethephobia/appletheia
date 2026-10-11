use appletheia::domain::UniqueValueError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamRoleAssigneeError {
    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),
}
