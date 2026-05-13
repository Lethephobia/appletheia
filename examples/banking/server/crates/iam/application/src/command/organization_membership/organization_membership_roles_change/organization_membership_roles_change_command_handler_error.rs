use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationMembership, OrganizationMembershipError,
};
use thiserror::Error;

/// Represents errors returned while changing organization membership roles.
#[derive(Debug, Error)]
pub enum OrganizationMembershipRolesChangeCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization membership repository failed")]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error("organization membership aggregate failed")]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("target organization membership was not found")]
    TargetOrganizationMembershipNotFound,

    #[error("organization is not found")]
    OrganizationNotFound,

    #[error("organization is removed")]
    OrganizationRemoved,
}
