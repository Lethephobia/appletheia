use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Organization, OrganizationError};
use thiserror::Error;

/// Represents errors returned while transferring organization ownership.
#[derive(Debug, Error)]
pub enum OrganizationOwnershipTransferCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization was not found")]
    OrganizationNotFound,
}
