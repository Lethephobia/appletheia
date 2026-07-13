use appletheia::domain::{AggregateStateError, ReferenceValuesError};
use thiserror::Error;

/// Describes why deposit state cannot be indexed.
#[derive(Debug, Error)]
pub enum DepositStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
}
