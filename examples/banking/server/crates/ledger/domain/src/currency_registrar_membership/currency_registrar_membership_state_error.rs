use appletheia::domain::{
    AggregateStateError, ReferenceValuesError, UniqueValueError, UniqueValuesError,
};
use thiserror::Error;

/// Describes why CurrencyRegistrarMembership state metadata cannot be produced.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarMembershipStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),
    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),
    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),
}
