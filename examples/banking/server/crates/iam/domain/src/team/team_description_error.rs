use thiserror::Error;

/// Describes why a team description cannot be validated.
#[derive(Debug, Error)]
pub enum TeamDescriptionError {
    #[error("team description must not be empty")]
    Empty,

    #[error("team description must be 280 characters or fewer")]
    TooLong,
}
