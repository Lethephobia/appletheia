use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandHashError {
    #[error("command hash must be 64 lowercase hex chars")]
    InvalidFormat,
}
