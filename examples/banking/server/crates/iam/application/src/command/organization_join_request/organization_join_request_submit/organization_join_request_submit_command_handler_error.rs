use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use appletheia::domain::{UniqueValueError, UniqueValuePartError};
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationJoinRequest, OrganizationJoinRequestError,
    OrganizationMembership,
};
use thiserror::Error;

/// Represents errors returned while submitting an organization join request.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestSubmitCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization join request repository failed")]
    OrganizationJoinRequestRepository(#[from] RepositoryError<OrganizationJoinRequest>),

    #[error("organization membership repository failed")]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error("organization join request aggregate failed")]
    OrganizationJoinRequest(#[from] OrganizationJoinRequestError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("unique value part is invalid")]
    UniqueValuePart(#[from] UniqueValuePartError),

    #[error("unique value is invalid")]
    UniqueValue(#[from] UniqueValueError),
}

impl Retryability for OrganizationJoinRequestSubmitCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::OrganizationJoinRequestRepository(error) => error.is_retryable(),
            Self::OrganizationMembershipRepository(error) => error.is_retryable(),
            Self::OrganizationJoinRequest(_) => false,
            Self::Organization(_) => false,
            Self::UniqueValuePart(_) => false,
            Self::UniqueValue(_) => false,
        }
    }
}
