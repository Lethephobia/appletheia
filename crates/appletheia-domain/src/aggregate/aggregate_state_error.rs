use thiserror::Error;

use super::UniqueValuesError;

/// Errors produced by framework-level aggregate-state operations.
#[derive(Debug, Error)]
pub enum AggregateStateError {
    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),
}
