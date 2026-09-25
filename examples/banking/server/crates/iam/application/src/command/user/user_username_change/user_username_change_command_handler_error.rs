use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use appletheia::domain::UniqueValueError;
use banking_iam_domain::{User, UserError};
use thiserror::Error;

/// Represents errors returned while changing a username.
#[derive(Debug, Error)]
pub enum UserUsernameChangeCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("unique value failed")]
    UniqueValue(#[from] UniqueValueError),
}

impl Retryability for UserUsernameChangeCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::UserRepository(error) => error.is_retryable(),
            Self::User(_) => false,
            Self::UniqueValue(_) => false,
        }
    }
}
