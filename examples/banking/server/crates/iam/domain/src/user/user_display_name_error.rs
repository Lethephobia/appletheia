use thiserror::Error;

/// Describes why a user display-name value is invalid.
#[derive(Debug, Error)]
pub enum UserDisplayNameError {
    #[error("display name must not be empty")]
    Empty,

    #[error("display name must be 50 characters or fewer")]
    TooLong,
}
