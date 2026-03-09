use thiserror::Error;

/// Errors that can occur when creating or advancing an aggregate version.
#[derive(Debug, Error)]
pub enum AggregateVersionError {
    #[error("aggregate version must be positive, got {0}")]
    NegativeValue(i64),

    #[error("aggregate version overflow")]
    Overflow,
}
