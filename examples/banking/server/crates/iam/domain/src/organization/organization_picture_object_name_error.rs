/// Describes why an organization picture object name cannot be validated.
#[derive(Debug, thiserror::Error)]
pub enum OrganizationPictureObjectNameError {
    #[error("organization picture object name is empty")]
    Empty,

    #[error("organization picture object name format is invalid")]
    InvalidFormat,
}
