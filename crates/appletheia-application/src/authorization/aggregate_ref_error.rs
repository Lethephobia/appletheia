use thiserror::Error;

/// Represents errors that can occur while building an `AggregateRef`.
#[derive(Debug, Error)]
pub enum AggregateRefError {
    #[error("aggregate id is missing")]
    MissingAggregateId,
}
