use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{TeamMembership, TeamMembershipError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipDemoteFromManagerCommandHandlerError {
    #[error(transparent)]
    TeamMembershipRepository(#[from] RepositoryError<TeamMembership>),

    #[error(transparent)]
    TeamMembership(#[from] TeamMembershipError),
}

impl Retryability for TeamMembershipDemoteFromManagerCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::TeamMembershipRepository(error) => error.is_retryable(),
            Self::TeamMembership(_) => false,
        }
    }
}
