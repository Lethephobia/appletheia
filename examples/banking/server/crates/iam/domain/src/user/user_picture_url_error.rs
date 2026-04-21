/// Describes why a user picture URL cannot be validated.
#[derive(Debug, thiserror::Error)]
pub enum UserPictureUrlError {
    #[error("user picture URL is invalid")]
    Parse(#[from] url::ParseError),
}
