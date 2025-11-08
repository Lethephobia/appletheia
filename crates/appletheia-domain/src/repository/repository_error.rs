use std::fmt::Debug;
use thiserror::Error;

use crate::AggregateState;
use crate::aggregate::{Aggregate, AggregateId, AggregateVersionError};
use crate::event::{EventIdError, EventPayload};
use crate::snapshot::SnapshotIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PersistenceErrorKind {
    Config,
    Conflict,
    Timeout,
    Io,
    Tls,
    Serialization,
    ConstraintViolation,
    PoolClosed,
    InvalidArgument,
    Protocol,
    WorkerCrashed,
    Unknown,
}

#[derive(Debug, Error)]
pub enum RepositoryError<A: Aggregate> {
    #[error("event id error: {0}")]
    EventId(#[source] EventIdError),

    #[error("aggregate id error: {0}")]
    AggregateId(#[source] <A::Id as AggregateId>::Error),

    #[error("aggregate version error: {0}")]
    AggregateVersion(#[source] AggregateVersionError),

    #[error("event payload error: {0}")]
    EventPayload(#[source] <A::EventPayload as EventPayload>::Error),

    #[error("snapshot id error: {0}")]
    SnapshotId(#[source] SnapshotIdError),

    #[error("state error: {0}")]
    AggregateState(#[source] <A::State as AggregateState>::Error),

    #[error("aggregate error: {0}")]
    Aggregate(#[source] A::Error),

    #[error("persistence error: {kind:?}, code: {code:?}")]
    Persistence {
        kind: PersistenceErrorKind,
        code: Option<String>,
    },
}
