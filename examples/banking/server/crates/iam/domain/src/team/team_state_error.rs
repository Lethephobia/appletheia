use appletheia::domain::{AggregateStateError, ReferenceValuesError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
}
