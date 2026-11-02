use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Team, TeamError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamNameChangeCommandHandlerError {
    #[error(transparent)]
    TeamRepository(#[from] RepositoryError<Team>),

    #[error(transparent)]
    Team(#[from] TeamError),
}

impl Retryability for TeamNameChangeCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::TeamRepository(error) => error.is_retryable(),
            Self::Team(_) => false,
        }
    }
}
