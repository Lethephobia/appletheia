use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{User, UserError};
use thiserror::Error;

/// Represents errors returned while readying a user profile.
#[derive(Debug, Error)]
pub enum UserProfileReadyCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("user was not found")]
    UserNotFound,

    #[error("user profile is not ready after readying")]
    UserProfileNotReady,
}
