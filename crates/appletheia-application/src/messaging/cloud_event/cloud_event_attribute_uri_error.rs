use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventAttributeUriError {
    #[error("invalid URI syntax")]
    InvalidUri,
}
