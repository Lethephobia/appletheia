use appletheia::application::{Retryability, repository::RepositoryError};
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationMembership, OrganizationMembershipError, Team,
    TeamError, TeamMembership, TeamMembershipError, TeamRoleAssigneeError, TeamRoleAssignment,
    TeamRoleAssignmentError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamRoleAssignmentAssignCommandHandlerError {
    #[error(transparent)]
    TeamRoleAssignmentRepository(#[from] RepositoryError<TeamRoleAssignment>),

    #[error(transparent)]
    TeamRoleAssignment(#[from] TeamRoleAssignmentError),

    #[error(transparent)]
    TeamRepository(#[from] RepositoryError<Team>),

    #[error(transparent)]
    Team(#[from] TeamError),

    #[error(transparent)]
    TeamMembershipRepository(#[from] RepositoryError<TeamMembership>),

    #[error(transparent)]
    TeamMembership(#[from] TeamMembershipError),

    #[error(transparent)]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error(transparent)]
    Organization(#[from] OrganizationError),

    #[error(transparent)]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error(transparent)]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error(transparent)]
    Assignee(#[from] TeamRoleAssigneeError),

    #[error("team inactive")]
    TeamInactive,

    #[error("assignee inactive")]
    AssigneeInactive,

    #[error("team mismatch")]
    TeamMismatch,

    #[error("already assigned")]
    AlreadyAssigned,

    #[error("organization inactive")]
    OrganizationInactive,

    #[error("organization mismatch")]
    OrganizationMismatch,
}

impl Retryability for TeamRoleAssignmentAssignCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::TeamRoleAssignmentRepository(error) => error.is_retryable(),
            Self::TeamRoleAssignment(_) => false,
            Self::TeamRepository(error) => error.is_retryable(),
            Self::Team(_) => false,
            Self::TeamMembershipRepository(error) => error.is_retryable(),
            Self::TeamMembership(_) => false,
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
            Self::OrganizationMembershipRepository(error) => error.is_retryable(),
            Self::OrganizationMembership(_) => false,
            Self::Assignee(_) => false,
            Self::TeamInactive => false,
            Self::AssigneeInactive => false,
            Self::TeamMismatch => false,
            Self::AlreadyAssigned => false,
            Self::OrganizationInactive => false,
            Self::OrganizationMismatch => false,
        }
    }
}
