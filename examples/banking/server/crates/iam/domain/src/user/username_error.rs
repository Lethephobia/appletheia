use thiserror::Error;

/// Describes why a username value is invalid.
#[derive(Debug, Error)]
pub enum UsernameError {
    #[error("username must not be empty")]
    Empty,

    #[error("username must be 32 characters or fewer")]
    TooLong,

    #[error("username may only contain lowercase ASCII letters, digits, and underscores")]
    InvalidCharacter,
}
