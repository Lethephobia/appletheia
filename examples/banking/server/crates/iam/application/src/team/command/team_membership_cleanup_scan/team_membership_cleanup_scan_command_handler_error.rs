use appletheia::application::Retryability;
use appletheia::application::repository::{ReferenceIndexLookupError, RepositoryError};
use banking_iam_domain::team_membership::{TeamMembership, TeamMembershipError};
use banking_iam_domain::team_membership_cleanup::{
    TeamMembershipCleanup, TeamMembershipCleanupError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipCleanupScanCommandHandlerError {
    #[error(transparent)]
    TeamMembershipRepository(#[from] RepositoryError<TeamMembership>),

    #[error(transparent)]
    TeamMembership(#[from] TeamMembershipError),

    #[error("team membership cleanup repository failed")]
    TeamMembershipCleanupRepository(#[from] RepositoryError<TeamMembershipCleanup>),

    #[error("reference index lookup failed")]
    ReferenceIndexLookup(#[from] ReferenceIndexLookupError),

    #[error("team membership cleanup aggregate failed")]
    TeamMembershipCleanup(#[from] TeamMembershipCleanupError),

    #[error("team membership cleanup is already completed")]
    AlreadyCompleted,

    #[error("membership scan is already complete")]
    ScanAlreadyCompleted,
}

impl Retryability for TeamMembershipCleanupScanCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::TeamMembershipRepository(error) => error.is_retryable(),
            Self::TeamMembership(_) => false,
            Self::TeamMembershipCleanupRepository(error) => error.is_retryable(),
            Self::ReferenceIndexLookup(error) => error.is_retryable(),
            Self::TeamMembershipCleanup(_)
            | Self::AlreadyCompleted
            | Self::ScanAlreadyCompleted => false,
        }
    }
}
