use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use appletheia::domain::UniqueValueError;
use banking_iam_domain::{Organization, OrganizationError};
use thiserror::Error;

/// Represents errors returned while creating an organization.
#[derive(Debug, Error)]
pub enum OrganizationCreateCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("unique value failed")]
    UniqueValue(#[from] UniqueValueError),
}

impl Retryability for OrganizationCreateCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
            Self::UniqueValue(_) => false,
        }
    }
}
