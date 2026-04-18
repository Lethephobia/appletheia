use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectChecksumValueError {
    #[error("object storage checksum value is empty")]
    Empty,
    #[error("object storage checksum value format is invalid")]
    InvalidFormat,
}
