use thiserror::Error;

/// Describes why a token program ID is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum TokenProgramIdError {
    #[error("token program ID cannot be empty")]
    Empty,

    #[error("token program ID has an invalid format")]
    InvalidFormat,
}
