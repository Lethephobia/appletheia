use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{TeamId, TeamStateError};

#[derive(Debug, Error)]
pub enum TeamError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TeamId>),

    #[error(transparent)]
    State(#[from] TeamStateError),

    #[error("team is already created")]
    AlreadyCreated,

    #[error("team has been removed")]
    Removed,
}
