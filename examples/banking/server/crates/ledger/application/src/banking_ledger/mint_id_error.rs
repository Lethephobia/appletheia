use thiserror::Error;

/// Describes why a mint ID is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum MintIdError {
    #[error("mint ID cannot be empty")]
    Empty,

    #[error("mint ID is too long")]
    TooLong,

    #[error("mint ID has an invalid format")]
    InvalidFormat,
}
