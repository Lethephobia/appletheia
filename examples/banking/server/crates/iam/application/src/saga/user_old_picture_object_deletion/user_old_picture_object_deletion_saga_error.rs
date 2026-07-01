use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserOldPictureObjectDeletionSagaError {
    #[error("failed to update saga instance")]
    SagaInstance(#[from] SagaInstanceError),

    #[error("unexpected user old picture object deletion saga event")]
    UnexpectedEvent,
}
