use thiserror::Error;

/// Describes why an email is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum EmailError {
    #[error("email cannot be empty")]
    Empty,

    #[error("email is too long")]
    TooLong,

    #[error("email cannot contain whitespace")]
    ContainsWhitespace,

    #[error("email must contain '@'")]
    MissingSeparator,

    #[error("email has an invalid format")]
    InvalidFormat,
}
