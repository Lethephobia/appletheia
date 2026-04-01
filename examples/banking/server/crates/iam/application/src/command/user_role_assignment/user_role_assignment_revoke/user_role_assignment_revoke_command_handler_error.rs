use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{UserRoleAssignment, UserRoleAssignmentError};
use thiserror::Error;

/// Represents errors returned while revoking a role assignment from a user.
#[derive(Debug, Error)]
pub enum UserRoleAssignmentRevokeCommandHandlerError {
    #[error("user role assignment repository failed")]
    UserRoleAssignmentRepository(#[from] RepositoryError<UserRoleAssignment>),

    #[error("user role assignment aggregate failed")]
    UserRoleAssignment(#[from] UserRoleAssignmentError),

    #[error("active user role assignment was not found")]
    UserRoleAssignmentNotFound,
}
