use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandNameOwnedError {
    #[error("command name is empty")]
    Empty,
    #[error("command name is too long")]
    TooLong,
    #[error("command name must be snake_case ascii: [a-z0-9_]")]
    InvalidFormat,
}
