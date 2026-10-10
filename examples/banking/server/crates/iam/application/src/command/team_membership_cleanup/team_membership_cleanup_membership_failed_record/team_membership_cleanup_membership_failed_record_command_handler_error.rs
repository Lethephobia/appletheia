use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_iam_domain::team_membership_cleanup::{
    TeamMembershipCleanup, TeamMembershipCleanupError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipCleanupMembershipFailedRecordCommandHandlerError {
    #[error("team membership cleanup repository failed")]
    TeamMembershipCleanupRepository(#[from] RepositoryError<TeamMembershipCleanup>),

    #[error("team membership cleanup aggregate failed")]
    TeamMembershipCleanup(#[from] TeamMembershipCleanupError),
}

impl Retryability for TeamMembershipCleanupMembershipFailedRecordCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::TeamMembershipCleanupRepository(error) => error.is_retryable(),
            Self::TeamMembershipCleanup(_) => false,
        }
    }
}
