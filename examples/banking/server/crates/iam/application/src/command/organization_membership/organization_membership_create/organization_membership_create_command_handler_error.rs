use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipError};
use thiserror::Error;

/// Represents errors returned while creating an organization membership.
#[derive(Debug, Error)]
pub enum OrganizationMembershipCreateCommandHandlerError {
    #[error("organization membership repository failed")]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error("organization membership aggregate failed")]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error("organization membership id is missing after create")]
    MissingOrganizationMembershipId,
}
