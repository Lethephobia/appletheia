use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum SignedObjectUploadUrlError {
    #[error("signed object storage upload url parse failed")]
    Parse(#[source] ParseError),
}
