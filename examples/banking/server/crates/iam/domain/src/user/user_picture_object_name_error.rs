/// Describes why a user picture object name cannot be validated.
#[derive(Debug, thiserror::Error)]
pub enum UserPictureObjectNameError {
    #[error("user picture object name is empty")]
    Empty,

    #[error("user picture object name format is invalid")]
    InvalidFormat,
}
