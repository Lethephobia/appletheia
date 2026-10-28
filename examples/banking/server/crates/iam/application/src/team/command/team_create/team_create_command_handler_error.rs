use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_iam_domain::{Organization, OrganizationError, Team, TeamError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamCreateCommandHandlerError {
    #[error(transparent)]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error(transparent)]
    TeamRepository(#[from] RepositoryError<Team>),

    #[error(transparent)]
    Organization(#[from] OrganizationError),

    #[error(transparent)]
    Team(#[from] TeamError),

    #[error("organization removed")]
    OrganizationRemoved,
}

impl Retryability for TeamCreateCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::TeamRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
            Self::Team(_) => false,
            Self::OrganizationRemoved => false,
        }
    }
}
