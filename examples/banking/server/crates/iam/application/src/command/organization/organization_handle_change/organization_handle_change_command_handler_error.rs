use appletheia::application::repository::RepositoryError;
use appletheia::domain::UniqueValueError;
use banking_iam_domain::{Organization, OrganizationError};
use thiserror::Error;

/// Represents errors returned while changing an organization handle.
#[derive(Debug, Error)]
pub enum OrganizationHandleChangeCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("unique value failed")]
    UniqueValue(#[from] UniqueValueError),
}
