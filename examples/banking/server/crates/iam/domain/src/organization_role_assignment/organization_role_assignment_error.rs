use super::{OrganizationRoleAssignmentId, OrganizationRoleAssignmentStateError};
use appletheia::domain::AggregateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationRoleAssignmentError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationRoleAssignmentId>),

    #[error(transparent)]
    State(#[from] OrganizationRoleAssignmentStateError),

    #[error("role assignment already exists")]
    AlreadyAssigned,

    #[error("role assignment is revoked")]
    Revoked,
}
