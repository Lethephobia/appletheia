use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventSourceError {
    #[error("source cannot be empty")]
    Empty,
    #[error("invalid URI syntax")]
    InvalidUri,
}
