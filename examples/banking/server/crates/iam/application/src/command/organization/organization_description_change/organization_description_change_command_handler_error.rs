use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Organization, OrganizationError};
use thiserror::Error;

/// Represents errors returned while changing an organization description.
#[derive(Debug, Error)]
pub enum OrganizationDescriptionChangeCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),
}

impl Retryability for OrganizationDescriptionChangeCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
        }
    }
}
