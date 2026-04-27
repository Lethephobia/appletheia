use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{User, UserError};
use thiserror::Error;

/// Represents errors returned while changing a user picture.
#[derive(Debug, Error)]
pub enum UserPictureChangeCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("user was not found")]
    UserNotFound,
}
