use thiserror::Error;

/// Describes why an organization display name cannot be validated.
#[derive(Debug, Error)]
pub enum OrganizationDisplayNameError {
    #[error("organization display name cannot be empty")]
    Empty,

    #[error("organization display name is too long")]
    TooLong,
}
