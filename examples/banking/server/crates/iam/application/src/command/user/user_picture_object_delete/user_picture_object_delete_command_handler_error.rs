use appletheia::application::Retryability;
use appletheia::application::object_storage::{ObjectDeleterError, ObjectNameError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserPictureObjectDeleteCommandHandlerError {
    #[error("object name is invalid")]
    ObjectName(#[from] ObjectNameError),

    #[error("object delete failed")]
    ObjectDeleter(#[from] ObjectDeleterError),
}

impl Retryability for UserPictureObjectDeleteCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::ObjectName(_) => false,
            Self::ObjectDeleter(error) => error.is_retryable(),
        }
    }
}
