use thiserror::Error;

/// Describes why a user picture URL cannot be validated.
#[derive(Debug, Error)]
pub enum UserPictureUrlError {
    #[error("user picture URL is invalid")]
    Parse(#[from] url::ParseError),
}
