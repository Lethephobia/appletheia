use appletheia::application::saga::SagaContextError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationOldPictureObjectDeletionSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),

    #[error("unexpected organization old picture object deletion saga event")]
    UnexpectedEvent,
}
