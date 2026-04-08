use thiserror::Error;

/// Describes why a user bio value is invalid.
#[derive(Debug, Error)]
pub enum UserBioError {
    #[error("bio must not be empty")]
    Empty,

    #[error("bio must be 280 characters or fewer")]
    TooLong,
}
