use appletheia::application::saga::SagaContextError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserOldPictureObjectDeletionSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),

    #[error("unexpected user old picture object deletion saga event")]
    UnexpectedEvent,
}
