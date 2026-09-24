use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrderingKeyError {
    #[error("ordering key cannot be empty")]
    Empty,
}
