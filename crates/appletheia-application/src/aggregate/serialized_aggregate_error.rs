use std::error::Error;

use super::SerializedAggregateStateError;

#[derive(Debug, thiserror::Error)]
pub enum SerializedAggregateError {
    #[error("aggregate type mismatch: expected {expected}, got {actual}")]
    AggregateTypeMismatch {
        expected: &'static str,
        actual: String,
    },

    #[error("aggregate id error: {0}")]
    AggregateId(#[source] Box<dyn Error + Send + Sync>),

    #[error(transparent)]
    SerializedAggregateState(#[from] SerializedAggregateStateError),
}
