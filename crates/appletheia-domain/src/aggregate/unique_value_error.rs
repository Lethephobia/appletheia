use thiserror::Error;

use super::UniqueValuePartError;

/// Errors that can occur when constructing a `UniqueValue`.
#[derive(Debug, Error)]
pub enum UniqueValueError {
    #[error("unique value must contain at least one part")]
    EmptyParts,

    #[error(transparent)]
    Part(#[from] UniqueValuePartError),
}
