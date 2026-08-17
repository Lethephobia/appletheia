use thiserror::Error;

/// Reports an invalid owned read model name.
#[derive(Debug, Error)]
pub enum ReadModelNameOwnedError {
    #[error("read model name is empty")]
    Empty,
    #[error("read model name is too long")]
    TooLong,
    #[error("read model name must be snake_case ascii: [a-z0-9_]")]
    InvalidFormat,
}
