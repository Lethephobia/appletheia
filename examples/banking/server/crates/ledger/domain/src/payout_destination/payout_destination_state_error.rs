use appletheia::domain::{AggregateStateError, ReferenceValuesError};
use thiserror::Error;

/// Describes why a payout destination state value cannot be handled.
#[derive(Debug, Error)]
pub enum PayoutDestinationStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
}
