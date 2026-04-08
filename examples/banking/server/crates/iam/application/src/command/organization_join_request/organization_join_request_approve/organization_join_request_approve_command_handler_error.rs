use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationJoinRequest, OrganizationJoinRequestError,
};
use thiserror::Error;

/// Represents errors returned while approving an organization join request.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestApproveCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization join request repository failed")]
    OrganizationJoinRequestRepository(#[from] RepositoryError<OrganizationJoinRequest>),

    #[error("organization join request aggregate failed")]
    OrganizationJoinRequest(#[from] OrganizationJoinRequestError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("target organization join request was not found")]
    TargetOrganizationJoinRequestNotFound,

    #[error("organization is not found")]
    OrganizationNotFound,

    #[error("organization is removed")]
    OrganizationRemoved,
}
