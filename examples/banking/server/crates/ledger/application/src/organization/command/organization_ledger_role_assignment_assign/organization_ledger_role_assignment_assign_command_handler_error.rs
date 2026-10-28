use appletheia::application::{Retryability, repository::RepositoryError};
use banking_iam_domain::{
    Organization, OrganizationError, OrganizationMembership, OrganizationMembershipError, Team,
    TeamError,
};
use banking_ledger_domain::OrganizationLedgerRoleAssigneeError;
use banking_ledger_domain::{
    OrganizationLedgerRoleAssignment, OrganizationLedgerRoleAssignmentError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationLedgerRoleAssignmentAssignCommandHandlerError {
    #[error(transparent)]
    OrganizationLedgerRoleAssignmentRepository(
        #[from] RepositoryError<OrganizationLedgerRoleAssignment>,
    ),

    #[error(transparent)]
    OrganizationLedgerRoleAssignment(#[from] OrganizationLedgerRoleAssignmentError),

    #[error(transparent)]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error(transparent)]
    Organization(#[from] OrganizationError),

    #[error(transparent)]
    OrganizationMembershipRepository(#[from] RepositoryError<OrganizationMembership>),

    #[error(transparent)]
    OrganizationMembership(#[from] OrganizationMembershipError),

    #[error(transparent)]
    TeamRepository(#[from] RepositoryError<Team>),

    #[error(transparent)]
    Team(#[from] TeamError),

    #[error(transparent)]
    Assignee(#[from] OrganizationLedgerRoleAssigneeError),

    #[error("organization inactive")]
    OrganizationInactive,

    #[error("assignee inactive")]
    AssigneeInactive,

    #[error("organization mismatch")]
    OrganizationMismatch,

    #[error("already assigned")]
    AlreadyAssigned,
}

impl Retryability for OrganizationLedgerRoleAssignmentAssignCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationLedgerRoleAssignmentRepository(error) => error.is_retryable(),
            Self::OrganizationLedgerRoleAssignment(_) => false,
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
            Self::OrganizationMembershipRepository(error) => error.is_retryable(),
            Self::OrganizationMembership(_) => false,
            Self::TeamRepository(error) => error.is_retryable(),
            Self::Team(_) => false,
            Self::Assignee(_) => false,
            Self::OrganizationInactive => false,
            Self::AssigneeInactive => false,
            Self::OrganizationMismatch => false,
            Self::AlreadyAssigned => false,
        }
    }
}
