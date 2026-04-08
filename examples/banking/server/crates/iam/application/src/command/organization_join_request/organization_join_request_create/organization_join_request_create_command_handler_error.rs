use appletheia::application::repository::RepositoryError;
use appletheia::domain::{AggregateId, UniqueValueError, UniqueValuePartError};
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationJoinRequest, OrganizationJoinRequestError,
    OrganizationMembership, UserId,
};
use thiserror::Error;

/// Represents errors returned while creating an organization join request.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestCreateCommandHandlerError {
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

    #[error("organization join request id is missing after request")]
    MissingOrganizationJoinRequestId,

    #[error("join requester principal must be available")]
    JoinRequesterRequiresPrincipal,

    #[error("join requester principal must be a user")]
    JoinRequesterRequiresUserPrincipal,

    #[error("join requester principal contains an invalid user id")]
    InvalidJoinRequesterUserId(#[source] <UserId as AggregateId>::Error),

    #[error("requester is already a member of the organization")]
    RequesterAlreadyMember,

    #[error("join request is already pending")]
    JoinRequestAlreadyRequested,

    #[error("organization is not found")]
    OrganizationNotFound,

    #[error("organization is removed")]
    OrganizationRemoved,
}
