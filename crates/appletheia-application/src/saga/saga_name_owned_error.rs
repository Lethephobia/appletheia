use thiserror::Error;

#[derive(Debug, Error)]
pub enum SagaNameOwnedError {
    #[error("saga name is empty")]
    Empty,
    #[error("saga name is too long")]
    TooLong,
    #[error("saga name must be snake_case ascii: [a-z0-9_]")]
    InvalidFormat,
}
