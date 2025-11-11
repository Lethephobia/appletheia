use thiserror::Error;

use appletheia_domain::{
    Aggregate, AggregateId, AggregateState, AggregateVersionError, SnapshotIdError,
};

#[derive(Debug, Error)]
pub enum PgSnapshotError<A: Aggregate> {
    #[error("aggregate id error: {0}")]
    AggregateId(#[source] <A::Id as AggregateId>::Error),

    #[error("snapshot id error: {0}")]
    SnapshotId(#[source] SnapshotIdError),

    #[error("aggregate version error: {0}")]
    AggregateVersion(#[source] AggregateVersionError),

    #[error("aggregate state error: {0}")]
    AggregateState(#[source] <A::State as AggregateState>::Error),
}
