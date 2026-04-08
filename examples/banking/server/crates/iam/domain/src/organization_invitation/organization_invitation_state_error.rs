use appletheia::domain::{
    AggregateStateError, UniqueValueError, UniqueValuePartError, UniqueValuesError,
};
use thiserror::Error;

/// Describes why an organization invitation state value cannot be handled.
#[derive(Debug, Error)]
pub enum OrganizationInvitationStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),

    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),

    #[error(transparent)]
    UniqueValuePart(#[from] UniqueValuePartError),
}
