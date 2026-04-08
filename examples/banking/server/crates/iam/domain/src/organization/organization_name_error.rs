use thiserror::Error;

/// Describes why an organization name cannot be validated.
#[derive(Debug, Error)]
pub enum OrganizationNameError {
    #[error("organization name cannot be empty")]
    Empty,

    #[error("organization name is too long")]
    TooLong,
}
