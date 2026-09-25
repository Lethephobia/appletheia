use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use appletheia::domain::{UniqueValueError, UniqueValuePartError};
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationMembership, OrganizationMembershipError, User,
    UserError,
};
use thiserror::Error;

/// Represents errors returned while creating an organization membership.
#[derive(Debug, Error)]
pub enum OrganizationMembershipCreateCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization membership repository failed")]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization membership aggregate failed")]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),

    #[error(transparent)]
    UniqueValuePart(#[from] UniqueValuePartError),
}

impl Retryability for OrganizationMembershipCreateCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::OrganizationMembershipRepository(error) => error.is_retryable(),
            Self::UserRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
            Self::OrganizationMembership(_) => false,
            Self::User(_) => false,
            Self::UniqueValue(_) => false,
            Self::UniqueValuePart(_) => false,
        }
    }
}
