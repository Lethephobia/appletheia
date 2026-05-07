use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationOldPictureObjectDeletionSagaError {
    #[error("unexpected organization old picture object deletion saga event")]
    UnexpectedEvent,
}
