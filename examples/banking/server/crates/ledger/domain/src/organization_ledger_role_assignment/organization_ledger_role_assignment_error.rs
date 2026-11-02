use super::{OrganizationLedgerRoleAssignmentId, OrganizationLedgerRoleAssignmentStateError};
use appletheia::domain::AggregateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationLedgerRoleAssignmentError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationLedgerRoleAssignmentId>),

    #[error(transparent)]
    State(#[from] OrganizationLedgerRoleAssignmentStateError),

    #[error("role assignment already exists")]
    AlreadyAssigned,

    #[error("role assignment is unassigned")]
    Unassigned,
}
