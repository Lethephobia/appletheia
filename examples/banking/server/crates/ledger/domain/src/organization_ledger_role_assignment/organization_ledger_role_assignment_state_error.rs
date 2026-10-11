use super::OrganizationLedgerRoleAssigneeError;
use appletheia::domain::{
    AggregateStateError, ReferenceValuesError, UniqueValueError, UniqueValuePartError,
    UniqueValuesError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationLedgerRoleAssignmentStateError {
    #[error(transparent)]
    Assignee(#[from] OrganizationLedgerRoleAssigneeError),

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
