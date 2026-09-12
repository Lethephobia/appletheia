use crate::aggregate::SerializedAggregateError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelationshipDerivationError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error("relationship handler failed: {0}")]
    Handler(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("relationship target does not match the declared relation")]
    TargetMismatch,
}
