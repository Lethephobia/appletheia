use std::error::Error;
use std::fmt::Debug;

use thiserror::Error;

use appletheia_domain::{Aggregate, AggregateType};

use crate::Retryability;
use crate::event::{EventReaderError, EventWriterError};
use crate::outbox::event::EventOutboxEnqueueError;
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

    #[error("event outbox enqueue error: {0}")]
    EventOutboxEnqueue(#[from] EventOutboxEnqueueError),

    #[error("event save hook error: {0}")]
    EventSaveHook(#[source] Box<dyn Error + Send + Sync>),

    #[error("snapshot writer error: {0}")]
    SnapshotWriter(#[from] SnapshotWriterError),
}

impl<A: Aggregate> Retryability for RepositoryError<A> {
    fn is_retryable(&self) -> bool {
        match self {
            Self::NotFound { .. } | Self::Aggregate(_) => false,
            Self::UniqueKeyReservationStore(error) => match error {
                UniqueKeyReservationStoreError::Conflict { .. }
                | UniqueKeyReservationStoreError::Persistence(_) => true,
                UniqueKeyReservationStoreError::NamespaceMismatch { .. }
                | UniqueKeyReservationStoreError::DuplicateKey { .. } => false,
            },
            Self::UniqueValueOwnerLookup(error) => match error {
                UniqueValueOwnerLookupError::OwnerAggregateId(_) => false,
                UniqueValueOwnerLookupError::Persistence(_) => true,
            },
            Self::ReferenceIndexStore(_) | Self::EventSaveHook(_) => true,
            Self::EventReader(error) => match error {
                EventReaderError::MappingFailed(_) | EventReaderError::NotInTransaction => false,
                EventReaderError::Persistence(_) => true,
            },
            Self::SnapshotReader(error) => match error {
                SnapshotReaderError::MappingFailed(_) | SnapshotReaderError::NotInTransaction => {
                    false
                }
                SnapshotReaderError::Persistence(_) => true,
            },
            Self::EventWriter(error) => match error {
                EventWriterError::NotInTransaction | EventWriterError::Json(_) => false,
                EventWriterError::Persistence(_) => true,
            },
            Self::EventOutboxEnqueue(error) => match error {
                EventOutboxEnqueueError::NotInTransaction => false,
                EventOutboxEnqueueError::Persistence(_) => true,
            },
            Self::SnapshotWriter(error) => match error {
                SnapshotWriterError::NotInTransaction | SnapshotWriterError::Json(_) => false,
                SnapshotWriterError::Persistence(_) => true,
            },
        }
    }
}
