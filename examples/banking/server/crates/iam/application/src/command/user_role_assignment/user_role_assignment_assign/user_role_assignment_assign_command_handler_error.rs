use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Role, RoleNameError, User, UserRoleAssignment, UserRoleAssignmentError};
use thiserror::Error;

/// Represents errors returned while assigning a role to a user.
#[derive(Debug, Error)]
pub enum UserRoleAssignmentAssignCommandHandlerError {
    #[error("role repository failed")]
    RoleRepository(#[from] RepositoryError<Role>),

    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user role assignment repository failed")]
    UserRoleAssignmentRepository(#[from] RepositoryError<UserRoleAssignment>),

    #[error("user role assignment aggregate failed")]
    UserRoleAssignment(#[from] UserRoleAssignmentError),

    #[error("role name is invalid")]
    RoleName(#[from] RoleNameError),

    #[error("role was not found")]
    RoleNotFound,

    #[error("user was not found")]
    UserNotFound,

    #[error("user role assignment id is missing after assign")]
    MissingUserRoleAssignmentId,
}
