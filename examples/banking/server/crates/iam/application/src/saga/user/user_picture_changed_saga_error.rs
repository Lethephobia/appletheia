use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserPictureChangedSagaError {
    #[error("unexpected user picture changed saga event")]
    UnexpectedEvent,
}
