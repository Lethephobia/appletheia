use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectUploadHeaderNameError {
    #[error("object storage upload header name is empty")]
    Empty,
    #[error("object storage upload header name format is invalid")]
    InvalidFormat,
}
