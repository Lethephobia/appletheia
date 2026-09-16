use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventExtensionNameError {
    #[error("extension names must be nonempty lowercase ASCII alphanumeric strings")]
    InvalidName,
    #[error("extension name is reserved")]
    ReservedName,
}
