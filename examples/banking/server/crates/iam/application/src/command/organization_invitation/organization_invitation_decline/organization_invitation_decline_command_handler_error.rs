use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationInvitation, OrganizationInvitationError,
};
use thiserror::Error;

/// Represents errors returned while declining an organization invitation.
#[derive(Debug, Error)]
pub enum OrganizationInvitationDeclineCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization invitation repository failed")]
    OrganizationInvitationRepository(#[from] RepositoryError<OrganizationInvitation>),

    #[error("organization invitation aggregate failed")]
    OrganizationInvitation(#[from] OrganizationInvitationError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),
}

impl Retryability for OrganizationInvitationDeclineCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::OrganizationInvitationRepository(error) => error.is_retryable(),
            Self::OrganizationInvitation(_) => false,
            Self::Organization(_) => false,
        }
    }
}
