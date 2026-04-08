use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationMembership, OrganizationMembershipError,
};
use thiserror::Error;

/// Represents errors returned while creating an organization membership.
#[derive(Debug, Error)]
pub enum OrganizationMembershipCreateCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization membership repository failed")]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error("organization membership aggregate failed")]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization membership id is missing after create")]
    MissingOrganizationMembershipId,

    #[error("organization is not found")]
    OrganizationNotFound,

    #[error("organization is removed")]
    OrganizationRemoved,
}
