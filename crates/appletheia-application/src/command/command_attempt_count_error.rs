use thiserror::Error;

/// Reports an invalid persisted command attempt count.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum CommandAttemptCountError {
    #[error("command attempt count must be positive")]
    Zero,

    #[error("command attempt count is out of range")]
    OutOfRange,
}
