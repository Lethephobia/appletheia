use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{User, UserError};
use thiserror::Error;

/// Represents errors returned while deactivating a user.
#[derive(Debug, Error)]
pub enum UserDeactivateCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("target user was not found")]
    TargetUserNotFound,
}
