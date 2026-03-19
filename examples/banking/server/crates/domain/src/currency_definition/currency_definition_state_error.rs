use appletheia::domain::{
    AggregateStateError, UniqueValueError, UniqueValuePartError, UniqueValuesError,
};
use thiserror::Error;

/// Describes why a currency-definition state value cannot be handled.
#[derive(Debug, Error)]
pub enum CurrencyDefinitionStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),

    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),

    #[error(transparent)]
    UniqueValuePart(#[from] UniqueValuePartError),
}
