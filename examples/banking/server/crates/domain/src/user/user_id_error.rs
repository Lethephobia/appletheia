use thiserror::Error;

/// Describes why a user ID is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum UserIdError {
    #[error("user id cannot be nil")]
    NilUuid,
}
