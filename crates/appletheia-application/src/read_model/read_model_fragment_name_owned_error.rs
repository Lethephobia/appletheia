use thiserror::Error;

/// Reports an invalid owned read model fragment name.
#[derive(Debug, Error)]
pub enum ReadModelFragmentNameOwnedError {
    #[error("read model fragment name is empty")]
    Empty,
    #[error("read model fragment name is too long")]
    TooLong,
    #[error("read model fragment name must be snake_case ascii: [a-z0-9_]")]
    InvalidFormat,
}
