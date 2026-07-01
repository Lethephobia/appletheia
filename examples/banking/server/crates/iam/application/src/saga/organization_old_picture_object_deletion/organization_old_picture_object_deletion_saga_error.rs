use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationOldPictureObjectDeletionSagaError {
    #[error("failed to update saga instance")]
    SagaInstance(#[from] SagaInstanceError),

    #[error("unexpected organization old picture object deletion saga event")]
    UnexpectedEvent,
}
