use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectContentTypeError {
    #[error("object storage content type is empty")]
    Empty,
    #[error("object storage content type format is invalid")]
    InvalidFormat,
}
