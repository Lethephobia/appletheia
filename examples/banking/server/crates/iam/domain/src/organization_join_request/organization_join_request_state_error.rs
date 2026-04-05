use appletheia::domain::{
    AggregateStateError, UniqueValueError, UniqueValuePartError, UniqueValuesError,
};
use thiserror::Error;

/// Describes why an organization join request state value cannot be handled.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),

    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),

    #[error(transparent)]
    UniqueValuePart(#[from] UniqueValuePartError),
}
