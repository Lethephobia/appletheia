use appletheia::domain::AggregateError;
use thiserror::Error;

use super::UserRoleAssignmentId;

/// Describes why a `UserRoleAssignment` aggregate operation failed.
#[derive(Debug, Error)]
pub enum UserRoleAssignmentError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<UserRoleAssignmentId>),

    #[error("user role assignment is already created")]
    AlreadyAssigned,
}
