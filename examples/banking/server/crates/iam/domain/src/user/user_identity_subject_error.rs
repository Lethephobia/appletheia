use thiserror::Error;

/// Describes why a user-identity subject value is invalid.
#[derive(Debug, Error)]
pub enum UserIdentitySubjectError {
    #[error("subject must not be empty")]
    Empty,

    #[error("subject must be 255 characters or fewer")]
    TooLong,
}
