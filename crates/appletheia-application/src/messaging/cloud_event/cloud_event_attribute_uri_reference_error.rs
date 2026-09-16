use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventAttributeUriReferenceError {
    #[error("invalid URI syntax")]
    InvalidUri,
}
