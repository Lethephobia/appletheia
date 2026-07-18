use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use appletheia::domain::UniqueValueError;
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationInvitation, OrganizationInvitationError, User,
    UserError,
};
use thiserror::Error;

/// Represents errors returned while issuing an organization invitation.
#[derive(Debug, Error)]
pub enum OrganizationInvitationIssueCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization invitation repository failed")]
    OrganizationInvitationRepository(#[from] RepositoryError<OrganizationInvitation>),

    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("organization invitation aggregate failed")]
    OrganizationInvitation(#[from] OrganizationInvitationError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("unique value failed")]
    UniqueValue(#[from] UniqueValueError),
}

impl Retryability for OrganizationInvitationIssueCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::OrganizationInvitationRepository(error) => error.is_retryable(),
            Self::UserRepository(error) => error.is_retryable(),
            Self::OrganizationInvitation(_) => false,
            Self::Organization(_) => false,
            Self::User(_) => false,
            Self::UniqueValue(_) => false,
        }
    }
}
