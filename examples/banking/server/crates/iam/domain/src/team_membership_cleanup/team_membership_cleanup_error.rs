use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    TeamMembershipCleanupCountError, TeamMembershipCleanupId, TeamMembershipCleanupStateError,
};

#[derive(Debug, Error)]
pub enum TeamMembershipCleanupError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TeamMembershipCleanupId>),

    #[error(transparent)]
    State(#[from] TeamMembershipCleanupStateError),

    #[error("team membership cleanup was already started")]
    AlreadyStarted,

    #[error("team membership cleanup is already completed")]
    AlreadyCompleted,

    #[error("membership scan is already complete")]
    ScanAlreadyCompleted,

    #[error("page cursor does not match the next expected cursor")]
    UnexpectedCursor,

    #[error("team membership cleanup has unfinished scanning or pending results")]
    NotReadyToComplete,

    #[error("all requested membership removal results have already been recorded")]
    NoPendingResults,

    #[error(transparent)]
    Count(#[from] TeamMembershipCleanupCountError),
}
