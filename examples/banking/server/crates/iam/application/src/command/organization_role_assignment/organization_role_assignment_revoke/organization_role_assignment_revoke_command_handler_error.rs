use appletheia::application::{Retryability, repository::RepositoryError};
use banking_iam_domain::{OrganizationRoleAssignment, OrganizationRoleAssignmentError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationRoleAssignmentRevokeCommandHandlerError {
    #[error(transparent)]
    OrganizationRoleAssignmentRepository(#[from] RepositoryError<OrganizationRoleAssignment>),

    #[error(transparent)]
    OrganizationRoleAssignment(#[from] OrganizationRoleAssignmentError),
}

impl Retryability for OrganizationRoleAssignmentRevokeCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRoleAssignmentRepository(error) => error.is_retryable(),
            Self::OrganizationRoleAssignment(_) => false,
        }
    }
}
