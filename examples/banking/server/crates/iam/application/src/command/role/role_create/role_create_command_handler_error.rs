use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Role, RoleError};
use thiserror::Error;

/// Represents errors returned while creating a role.
#[derive(Debug, Error)]
pub enum RoleCreateCommandHandlerError {
    #[error("role repository failed")]
    RoleRepository(#[from] RepositoryError<Role>),

    #[error("role aggregate failed")]
    Role(#[from] RoleError),

    #[error("role aggregate id is missing after create")]
    MissingRoleId,
}
