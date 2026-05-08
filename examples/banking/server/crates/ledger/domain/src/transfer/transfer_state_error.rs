use appletheia::domain::{AggregateStateError, ReferenceValuesError};
use thiserror::Error;

/// Describes why a transfer state value cannot be handled.
#[derive(Debug, Error)]
pub enum TransferStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
}
