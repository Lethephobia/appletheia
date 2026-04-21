use appletheia::application::object_storage::{ObjectNameError, ObjectUploadSignerError};
use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{User, UserError, UserPictureObjectNameError};
use thiserror::Error;

/// Represents errors returned while preparing a user-picture upload.
#[derive(Debug, Error)]
pub enum UserPictureUploadPrepareCommandHandlerError {
    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("picture object name generation failed")]
    PictureObjectName(#[from] UserPictureObjectNameError),

    #[error("object storage object name generation failed")]
    ObjectName(#[from] ObjectNameError),

    #[error("object upload signer failed")]
    ObjectUploadSigner(#[from] ObjectUploadSignerError),

    #[error("user was not found")]
    UserNotFound,

    #[error("inactive users cannot prepare picture uploads")]
    UserInactive,

    #[error("removed users cannot prepare picture uploads")]
    UserRemoved,

    #[error("picture content length exceeds the configured maximum")]
    ContentLengthTooLarge,

    #[error("picture content type is not allowed")]
    ContentTypeNotAllowed,
}
