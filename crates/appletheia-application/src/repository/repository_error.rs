use std::fmt::Debug;
use thiserror::Error;

use appletheia_domain::{Aggregate, AggregateState};

use crate::event::{EventReaderError, EventWriterError};
use crate::snapshot::{SnapshotReaderError, SnapshotWriterError};

use super::UniqueKeyReservationStoreError;

#[derive(Debug, Error)]
pub enum RepositoryError<A: Aggregate> {
    #[error("aggregate error: {0}")]
    Aggregate(#[source] A::Error),

    #[error("aggregate state error: {0}")]
    State(#[source] <A::State as AggregateState>::Error),

    #[error("unique key reservation store error: {0}")]
    UniqueKeyReservationStore(#[from] UniqueKeyReservationStoreError),

    #[error("event reader error: {0}")]
    EventReader(#[from] EventReaderError),

    #[error("snapshot reader error: {0}")]
    SnapshotReader(#[from] SnapshotReaderError),

    #[error("event writer error: {0}")]
    EventWriter(#[from] EventWriterError),

    #[error("snapshot writer error: {0}")]
    SnapshotWriter(#[from] SnapshotWriterError),
}
