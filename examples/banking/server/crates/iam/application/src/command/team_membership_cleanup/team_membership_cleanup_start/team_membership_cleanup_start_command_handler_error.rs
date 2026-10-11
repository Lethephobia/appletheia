use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_iam_domain::team_membership_cleanup::{
    TeamMembershipCleanup, TeamMembershipCleanupError,
};
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipCleanupStartCommandHandlerError {
    #[error(transparent)]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error(transparent)]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error("organization membership is not removed")]
    OrganizationMembershipNotRemoved,

    #[error("team membership cleanup repository failed")]
    TeamMembershipCleanupRepository(#[from] RepositoryError<TeamMembershipCleanup>),

    #[error("team membership cleanup aggregate failed")]
    TeamMembershipCleanup(#[from] TeamMembershipCleanupError),
}

impl Retryability for TeamMembershipCleanupStartCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationMembershipRepository(error) => error.is_retryable(),
            Self::OrganizationMembership(_) | Self::OrganizationMembershipNotRemoved => false,
            Self::TeamMembershipCleanupRepository(error) => error.is_retryable(),
            Self::TeamMembershipCleanup(_) => false,
        }
    }
}
