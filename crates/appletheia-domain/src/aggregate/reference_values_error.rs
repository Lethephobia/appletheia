use thiserror::Error;
use uuid::Uuid;

/// Errors returned when building `ReferenceValues`.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ReferenceValuesError {
    #[error("reference values must not be empty")]
    Empty,

    #[error("duplicate reference value: {value}")]
    DuplicateValue { value: Uuid },
}
