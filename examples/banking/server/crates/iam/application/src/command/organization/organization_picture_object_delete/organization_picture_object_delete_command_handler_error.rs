use appletheia::application::Retryability;

use appletheia::application::object_storage::{ObjectDeleterError, ObjectNameError};
use thiserror::Error;

/// Represents errors returned while deleting an organization picture object.
#[derive(Debug, Error)]
pub enum OrganizationPictureObjectDeleteCommandHandlerError {
    #[error("object name is invalid")]
    ObjectName(#[from] ObjectNameError),

    #[error("object delete failed")]
    ObjectDeleter(#[from] ObjectDeleterError),
}

impl Retryability for OrganizationPictureObjectDeleteCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::ObjectName(_) => false,
            Self::ObjectDeleter(error) => error.is_retryable(),
        }
    }
}
