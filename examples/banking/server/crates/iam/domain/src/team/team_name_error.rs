use thiserror::Error;

/// Describes why a team name cannot be validated.
#[derive(Debug, Error)]
pub enum TeamNameError {
    #[error("team name cannot be empty")]
    Empty,

    #[error("team name is too long")]
    TooLong,
}
