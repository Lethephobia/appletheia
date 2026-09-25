use appletheia::domain::{
    AggregateStateError, ReferenceValuesError, UniqueValueError, UniqueValuePartError,
    UniqueValuesError,
};
use thiserror::Error;

/// Describes why an currency registrar join request state value cannot be handled.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestStateError {
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
