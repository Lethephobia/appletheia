use thiserror::Error;

use crate::event::EventFeedReaderError;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::{ProjectionCheckpointStoreError, ProjectorProcessedEventStoreError};

#[derive(Debug, Error)]
pub enum ProjectorRebuilderError {
    #[error("event feed reader failed: {0}")]
    EventFeedReader(#[from] EventFeedReaderError),

    #[error("checkpoint store failed: {0}")]
    CheckpointStore(#[from] ProjectionCheckpointStoreError),

    #[error("processed event store failed: {0}")]
    ProcessedEventStore(#[from] ProjectorProcessedEventStoreError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("projector definition failed")]
    Definition(#[source] Box<dyn std::error::Error + Send + Sync>),
}
