use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{OrganizationInvitation, OrganizationInvitationError};
use thiserror::Error;

/// Represents errors returned while accepting an organization invitation.
#[derive(Debug, Error)]
pub enum OrganizationInvitationAcceptCommandHandlerError {
    #[error("organization invitation repository failed")]
    OrganizationInvitationRepository(#[from] RepositoryError<OrganizationInvitation>),

    #[error("organization invitation aggregate failed")]
    OrganizationInvitation(#[from] OrganizationInvitationError),

    #[error("target organization invitation was not found")]
    TargetOrganizationInvitationNotFound,
}
