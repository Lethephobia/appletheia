use thiserror::Error;

/// Reports an invalid owned read model part name.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ReadModelPartNameOwnedError {
    #[error("read model part name is empty")]
    Empty,
    #[error("read model part name is too long")]
    TooLong,
    #[error("read model part name must be snake_case ascii: [a-z0-9_]")]
    InvalidFormat,
}
