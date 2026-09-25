use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationJoinRequest, OrganizationJoinRequestError,
};
use thiserror::Error;

/// Represents errors returned while rejecting an organization join request.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestRejectCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization join request repository failed")]
    OrganizationJoinRequestRepository(#[from] RepositoryError<OrganizationJoinRequest>),

    #[error("organization join request aggregate failed")]
    OrganizationJoinRequest(#[from] OrganizationJoinRequestError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),
}

impl Retryability for OrganizationJoinRequestRejectCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::OrganizationJoinRequestRepository(error) => error.is_retryable(),
            Self::OrganizationJoinRequest(_) => false,
            Self::Organization(_) => false,
        }
    }
}
