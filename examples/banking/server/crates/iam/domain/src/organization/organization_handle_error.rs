use thiserror::Error;

/// Describes why an organization handle cannot be validated.
#[derive(Debug, Error)]
pub enum OrganizationHandleError {
    #[error("organization handle cannot be empty")]
    Empty,

    #[error("organization handle is too long")]
    TooLong,

    #[error("organization handle contains an invalid character")]
    InvalidCharacter,
}
