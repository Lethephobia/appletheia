use appletheia::application::Retryability;

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
}

impl Retryability for UserPictureUploadPrepareCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::UserRepository(error) => error.is_retryable(),
            Self::User(_) => false,
            Self::PictureObjectName(_) => false,
            Self::ObjectName(_) => false,
            Self::ObjectUploadSigner(error) => error.is_retryable(),
        }
    }
}
