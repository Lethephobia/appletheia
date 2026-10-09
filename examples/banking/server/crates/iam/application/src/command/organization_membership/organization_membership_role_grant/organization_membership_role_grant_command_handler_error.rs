use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationMembership, OrganizationMembershipError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationMembershipRoleGrantCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization membership repository failed")]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization membership aggregate failed")]
    OrganizationMembership(#[from] OrganizationMembershipError),
}

impl Retryability for OrganizationMembershipRoleGrantCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::OrganizationMembershipRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
            Self::OrganizationMembership(_) => false,
        }
    }
}
