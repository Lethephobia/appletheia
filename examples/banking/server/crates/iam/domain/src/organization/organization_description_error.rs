use thiserror::Error;

/// Describes why an organization description cannot be validated.
#[derive(Debug, Error)]
pub enum OrganizationDescriptionError {
    #[error("organization description must not be empty")]
    Empty,

    #[error("organization description must be 280 characters or fewer")]
    TooLong,
}
