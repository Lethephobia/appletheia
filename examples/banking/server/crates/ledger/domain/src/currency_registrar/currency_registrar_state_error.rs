use appletheia::domain::{
    AggregateStateError, ReferenceValuesError, UniqueValueError, UniqueValuesError,
};
use thiserror::Error;

/// Describes why CurrencyRegistrar state metadata cannot be produced.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),
    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),
    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),
}
