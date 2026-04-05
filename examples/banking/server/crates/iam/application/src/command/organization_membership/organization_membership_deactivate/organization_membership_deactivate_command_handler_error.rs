use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipError};
use thiserror::Error;

/// Represents errors returned while deactivating an organization membership.
#[derive(Debug, Error)]
pub enum OrganizationMembershipDeactivateCommandHandlerError {
    #[error("organization membership repository failed")]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error("organization membership aggregate failed")]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error("target organization membership was not found")]
    TargetOrganizationMembershipNotFound,
}
