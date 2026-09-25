use appletheia::domain::{
    AggregateStateError, ReferenceValuesError, UniqueValueError, UniqueValuesError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenBindingStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),
    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),
    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),
}
