use std::error::Error;
use std::fmt::Debug;

use thiserror::Error;

use appletheia_domain::{Aggregate, AggregateState, AggregateType};

use crate::event::{EventReaderError, EventWriterError};
use crate::snapshot::{SnapshotReaderError, SnapshotWriterError};

use super::{
    ReferenceIndexStoreError, UniqueKeyReservationStoreError, UniqueValueOwnerLookupError,
};

#[derive(Debug, Error)]
pub enum RepositoryError<A: Aggregate> {
    #[error("aggregate not found: {aggregate_type} {aggregate_id:?}")]
    NotFound {
        aggregate_type: AggregateType,
        aggregate_id: A::Id,
    },

    #[error("aggregate error: {0}")]
    Aggregate(#[source] A::Error),

    #[error("aggregate state error: {0}")]
    State(#[source] <A::State as AggregateState>::Error),

    #[error("unique key reservation store error: {0}")]
    UniqueKeyReservationStore(#[from] UniqueKeyReservationStoreError),

    #[error("unique value owner lookup error: {0}")]
    UniqueValueOwnerLookup(#[from] UniqueValueOwnerLookupError),

    #[error("reference index store error: {0}")]
    ReferenceIndexStore(#[from] ReferenceIndexStoreError),

    #[error("event reader error: {0}")]
    EventReader(#[from] EventReaderError),

    #[error("snapshot reader error: {0}")]
    SnapshotReader(#[from] SnapshotReaderError),

    #[error("event writer error: {0}")]
    EventWriter(#[from] EventWriterError),

    #[error("event save hook error: {0}")]
    EventSaveHook(#[source] Box<dyn Error + Send + Sync>),

    #[error("snapshot writer error: {0}")]
    SnapshotWriter(#[from] SnapshotWriterError),
}
