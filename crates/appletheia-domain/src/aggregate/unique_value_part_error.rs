use thiserror::Error;

/// Errors that can occur when constructing a `UniqueValuePart`.
#[derive(Debug, Error)]
pub enum UniqueValuePartError {
    #[error("unique key part must not be empty")]
    Empty,
}
