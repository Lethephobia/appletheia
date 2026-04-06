use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationInvitation, OrganizationInvitationError,
};
use thiserror::Error;

/// Represents errors returned while accepting an organization invitation.
#[derive(Debug, Error)]
pub enum OrganizationInvitationAcceptCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization invitation repository failed")]
    OrganizationInvitationRepository(#[from] RepositoryError<OrganizationInvitation>),

    #[error("organization invitation aggregate failed")]
    OrganizationInvitation(#[from] OrganizationInvitationError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("target organization invitation was not found")]
    TargetOrganizationInvitationNotFound,

    #[error("organization is not found")]
    OrganizationNotFound,

    #[error("organization is removed")]
    OrganizationRemoved,
}
