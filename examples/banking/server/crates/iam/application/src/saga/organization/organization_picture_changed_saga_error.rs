use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationPictureChangedSagaError {
    #[error("unexpected organization picture changed saga event")]
    UnexpectedEvent,
}
