use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Organization, OrganizationError};
use thiserror::Error;

/// Represents errors returned while removing an organization.
#[derive(Debug, Error)]
pub enum OrganizationRemoveCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization was not found")]
    OrganizationNotFound,
}
