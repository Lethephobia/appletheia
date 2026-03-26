use thiserror::Error;

/// Describes why a role name value is invalid.
#[derive(Debug, Error)]
pub enum RoleNameError {
    #[error("role name must not be empty")]
    Empty,

    #[error("role name must be 64 characters or fewer")]
    TooLong,

    #[error("role name may only contain lowercase ASCII letters, digits, and underscores")]
    InvalidCharacter,
}
