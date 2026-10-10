use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{TeamMembershipId, TeamMembershipStateError};

#[derive(Debug, Error)]
pub enum TeamMembershipError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TeamMembershipId>),

    #[error(transparent)]
    State(#[from] TeamMembershipStateError),

    #[error("team membership is already created")]
    AlreadyCreated,

    #[error("team membership has been removed")]
    Removed,
}
