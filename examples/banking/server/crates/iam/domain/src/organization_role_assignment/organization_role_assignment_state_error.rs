use super::OrganizationRoleAssigneeError;
use appletheia::domain::{
    AggregateStateError, ReferenceValuesError, UniqueValueError, UniqueValuePartError,
    UniqueValuesError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationRoleAssignmentStateError {
    #[error(transparent)]
    Assignee(#[from] OrganizationRoleAssigneeError),

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
