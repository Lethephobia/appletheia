use thiserror::Error;

/// Describes why a username is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum UsernameError {
    #[error("username cannot be empty")]
    Empty,

    #[error("username is too long")]
    TooLong,
}
