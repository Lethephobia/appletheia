use appletheia::application::{Retryability, repository::RepositoryError};
use banking_iam_domain::{TeamRoleAssignment, TeamRoleAssignmentError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamRoleAssignmentUnassignCommandHandlerError {
    #[error(transparent)]
    TeamRoleAssignmentRepository(#[from] RepositoryError<TeamRoleAssignment>),

    #[error(transparent)]
    TeamRoleAssignment(#[from] TeamRoleAssignmentError),
}

impl Retryability for TeamRoleAssignmentUnassignCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::TeamRoleAssignmentRepository(error) => error.is_retryable(),
            Self::TeamRoleAssignment(_) => false,
        }
    }
}
