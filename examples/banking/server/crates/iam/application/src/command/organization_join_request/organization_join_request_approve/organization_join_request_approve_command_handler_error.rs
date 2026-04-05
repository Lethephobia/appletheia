use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{OrganizationJoinRequest, OrganizationJoinRequestError};
use thiserror::Error;

/// Represents errors returned while approving an organization join request.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestApproveCommandHandlerError {
    #[error("organization join request repository failed")]
    OrganizationJoinRequestRepository(#[from] RepositoryError<OrganizationJoinRequest>),

    #[error("organization join request aggregate failed")]
    OrganizationJoinRequest(#[from] OrganizationJoinRequestError),

    #[error("target organization join request was not found")]
    TargetOrganizationJoinRequestNotFound,
}
