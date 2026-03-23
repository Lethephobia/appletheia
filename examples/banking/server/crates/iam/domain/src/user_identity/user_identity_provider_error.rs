use thiserror::Error;

/// Describes why a user-identity provider value is invalid.
#[derive(Debug, Error)]
pub enum UserIdentityProviderError {
    #[error("provider must not be empty")]
    Empty,

    #[error("provider must be 255 characters or fewer")]
    TooLong,
}
