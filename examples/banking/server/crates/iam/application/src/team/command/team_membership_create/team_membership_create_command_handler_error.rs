use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use appletheia::domain::{UniqueValueError, UniqueValuePartError};
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationMembership, OrganizationMembershipError, Team,
    TeamError, TeamMembership, TeamMembershipError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipCreateCommandHandlerError {
    #[error(transparent)]
    TeamRepository(#[from] RepositoryError<Team>),

    #[error(transparent)]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error(transparent)]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error(transparent)]
    TeamMembershipRepository(#[from] RepositoryError<TeamMembership>),

    #[error(transparent)]
    Team(#[from] TeamError),

    #[error(transparent)]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error(transparent)]
    Organization(#[from] OrganizationError),

    #[error(transparent)]
    TeamMembership(#[from] TeamMembershipError),

    #[error(transparent)]
    UniqueValue(#[from] UniqueValueError),

    #[error(transparent)]
    UniqueValuePart(#[from] UniqueValuePartError),

    #[error("team removed")]
    TeamRemoved,

    #[error("organization membership removed")]
    OrganizationMembershipRemoved,

    #[error("organization mismatch")]
    OrganizationMismatch,

    #[error("organization removed")]
    OrganizationRemoved,

    #[error("already member")]
    AlreadyMember,
}

impl Retryability for TeamMembershipCreateCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::TeamRepository(error) => error.is_retryable(),
            Self::OrganizationMembershipRepository(error) => error.is_retryable(),
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::TeamMembershipRepository(error) => error.is_retryable(),
            Self::Team(_) => false,
            Self::OrganizationMembership(_) => false,
            Self::Organization(_) => false,
            Self::TeamMembership(_) => false,
            Self::UniqueValue(_) => false,
            Self::UniqueValuePart(_) => false,
            Self::TeamRemoved => false,
            Self::OrganizationMembershipRemoved => false,
            Self::OrganizationMismatch => false,
            Self::OrganizationRemoved => false,
            Self::AlreadyMember => false,
        }
    }
}
