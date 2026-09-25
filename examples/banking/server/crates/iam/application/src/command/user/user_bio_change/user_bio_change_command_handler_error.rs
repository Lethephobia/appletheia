use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{User, UserError};
use thiserror::Error;

/// Represents errors returned while changing a user bio.
#[derive(Debug, Error)]
pub enum UserBioChangeCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),
}

impl Retryability for UserBioChangeCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::UserRepository(error) => error.is_retryable(),
            Self::User(_) => false,
        }
    }
}
