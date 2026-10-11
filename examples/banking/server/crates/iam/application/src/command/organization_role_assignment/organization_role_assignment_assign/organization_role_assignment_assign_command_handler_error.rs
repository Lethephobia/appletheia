use appletheia::application::{Retryability, repository::RepositoryError};
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationMembership, OrganizationMembershipError,
    OrganizationRoleAssigneeError, OrganizationRoleAssignment, OrganizationRoleAssignmentError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationRoleAssignmentAssignCommandHandlerError {
    #[error(transparent)]
    OrganizationRoleAssignmentRepository(#[from] RepositoryError<OrganizationRoleAssignment>),

    #[error(transparent)]
    OrganizationRoleAssignment(#[from] OrganizationRoleAssignmentError),

    #[error(transparent)]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error(transparent)]
    Organization(#[from] OrganizationError),

    #[error(transparent)]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error(transparent)]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error(transparent)]
    Assignee(#[from] OrganizationRoleAssigneeError),

    #[error("organization inactive")]
    OrganizationInactive,

    #[error("assignee inactive")]
    AssigneeInactive,

    #[error("organization mismatch")]
    OrganizationMismatch,

    #[error("already assigned")]
    AlreadyAssigned,
}

impl Retryability for OrganizationRoleAssignmentAssignCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRoleAssignmentRepository(error) => error.is_retryable(),
            Self::OrganizationRoleAssignment(_) => false,
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
            Self::OrganizationMembershipRepository(error) => error.is_retryable(),
            Self::OrganizationMembership(_) => false,
            Self::Assignee(_) => false,
            Self::OrganizationInactive => false,
            Self::AssigneeInactive => false,
            Self::OrganizationMismatch => false,
            Self::AlreadyAssigned => false,
        }
    }
}
