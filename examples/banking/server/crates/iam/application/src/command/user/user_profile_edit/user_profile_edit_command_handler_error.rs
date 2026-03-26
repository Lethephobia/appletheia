use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{User, UserError};
use thiserror::Error;

/// Represents errors returned while editing a user profile.
#[derive(Debug, Error)]
pub enum UserProfileEditCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("user was not found")]
    UserNotFound,
}
