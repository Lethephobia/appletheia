use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipError};
use thiserror::Error;

/// Represents errors returned while removing an organization membership.
#[derive(Debug, Error)]
pub enum OrganizationMembershipRemoveCommandHandlerError {
    #[error("organization membership repository failed")]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error("organization membership aggregate failed")]
    OrganizationMembership(#[from] OrganizationMembershipError),
}

impl Retryability for OrganizationMembershipRemoveCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationMembershipRepository(error) => error.is_retryable(),
            Self::OrganizationMembership(_) => false,
        }
    }
}
