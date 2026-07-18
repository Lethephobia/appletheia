use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Organization, OrganizationError, User, UserError};
use thiserror::Error;

/// Represents errors returned while granting an organization membership to a user.
#[derive(Debug, Error)]
pub enum UserOrganizationMembershipGrantCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("user aggregate failed")]
    User(#[from] UserError),
}

impl Retryability for UserOrganizationMembershipGrantCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::UserRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
            Self::User(_) => false,
        }
    }
}
