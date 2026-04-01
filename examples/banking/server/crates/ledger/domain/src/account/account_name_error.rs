use thiserror::Error;

/// Describes why an account name is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum AccountNameError {
    #[error("account name cannot be empty")]
    Empty,

    #[error("account name is too long")]
    TooLong,
}
