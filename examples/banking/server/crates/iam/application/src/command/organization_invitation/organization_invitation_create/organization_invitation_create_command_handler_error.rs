use appletheia::application::repository::RepositoryError;
use appletheia::domain::{AggregateId, UniqueValueError, UniqueValuePartError};
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationInvitation, OrganizationInvitationError,
    OrganizationMembership, UserId,
};
use thiserror::Error;

/// Represents errors returned while issuing an organization invitation.
#[derive(Debug, Error)]
pub enum OrganizationInvitationIssueCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization invitation repository failed")]
    OrganizationInvitationRepository(#[from] RepositoryError<OrganizationInvitation>),

    #[error("organization membership repository failed")]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error("organization invitation aggregate failed")]
    OrganizationInvitation(#[from] OrganizationInvitationError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("unique value part is invalid")]
    UniqueValuePart(#[from] UniqueValuePartError),

    #[error("unique value is invalid")]
    UniqueValue(#[from] UniqueValueError),

    #[error("organization invitation id is missing after issue")]
    MissingOrganizationInvitationId,

    #[error("invitation issuer principal must be available")]
    InvitationIssuerRequiresPrincipal,

    #[error("invitation issuer principal must be a user")]
    InvitationIssuerRequiresUserPrincipal,

    #[error("invitation issuer principal contains an invalid user id")]
    InvalidInvitationIssuerUserId(#[source] <UserId as AggregateId>::Error),

    #[error("invitee is already a member of the organization")]
    InviteeAlreadyMember,

    #[error("organization is not found")]
    OrganizationNotFound,

    #[error("organization is removed")]
    OrganizationRemoved,
}
