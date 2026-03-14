use thiserror::Error;

use super::UniqueValue;

/// Errors that can occur when constructing `UniqueValues`.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum UniqueValuesError {
    #[error("unique values must contain at least one value")]
    Empty,

    #[error("duplicate unique value: {value}")]
    DuplicateValue { value: UniqueValue },
}
