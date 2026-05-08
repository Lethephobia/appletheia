use appletheia::domain::{
    AggregateStateError, ReferenceValuesError, UniqueValueError, UniqueValuePartError,
    UniqueValuesError,
};
use thiserror::Error;

/// Describes why a currency state value cannot be handled.
#[derive(Debug, Error)]
pub enum CurrencyStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),

    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),

    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),

    #[error(transparent)]
    UniqueValuePart(#[from] UniqueValuePartError),
}
