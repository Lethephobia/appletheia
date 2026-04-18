use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectUploadHeaderValueError {
    #[error("object storage upload header value format is invalid")]
    InvalidFormat,
}
