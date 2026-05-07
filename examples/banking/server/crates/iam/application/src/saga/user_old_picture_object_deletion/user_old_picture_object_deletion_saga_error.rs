use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserOldPictureObjectDeletionSagaError {
    #[error("unexpected user old picture object deletion saga event")]
    UnexpectedEvent,
}
