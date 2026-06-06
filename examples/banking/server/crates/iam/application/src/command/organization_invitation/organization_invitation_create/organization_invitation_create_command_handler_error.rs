use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationInvitation, OrganizationInvitationError, User,
    UserError,
};
use thiserror::Error;

/// Represents errors returned while issuing an organization invitation.
#[derive(Debug, Error)]
pub enum OrganizationInvitationIssueCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization invitation repository failed")]
    OrganizationInvitationRepository(#[from] RepositoryError<OrganizationInvitation>),

    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("organization invitation aggregate failed")]
    OrganizationInvitation(#[from] OrganizationInvitationError),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("organization is not found")]
    OrganizationNotFound,

    #[error("invitee is not found")]
    InviteeNotFound,
}
