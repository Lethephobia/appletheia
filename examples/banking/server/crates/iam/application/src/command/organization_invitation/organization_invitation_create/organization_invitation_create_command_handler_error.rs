use appletheia::application::repository::RepositoryError;
use appletheia::domain::{UniqueValueError, UniqueValuePartError};
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationInvitation, OrganizationInvitationError,
    OrganizationMembership,
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

    #[error("organization is not found")]
    OrganizationNotFound,
}
