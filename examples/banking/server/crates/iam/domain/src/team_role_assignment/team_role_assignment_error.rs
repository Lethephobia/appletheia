use super::{TeamRoleAssignmentId, TeamRoleAssignmentStateError};
use appletheia::domain::AggregateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamRoleAssignmentError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TeamRoleAssignmentId>),

    #[error(transparent)]
    State(#[from] TeamRoleAssignmentStateError),

    #[error("role assignment already exists")]
    AlreadyAssigned,

    #[error("role assignment is revoked")]
    Revoked,
}
