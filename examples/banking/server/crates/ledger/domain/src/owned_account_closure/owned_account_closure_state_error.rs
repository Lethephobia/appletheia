use appletheia::domain::{AggregateStateError, ReferenceValuesError};
use thiserror::Error;

/// Describes why an owned account closure state value cannot be handled.
#[derive(Debug, Error)]
pub enum OwnedAccountClosureStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
}
