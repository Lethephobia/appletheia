use thiserror::Error;

use crate::outbox::read_model_fragment_change::ReadModelFragmentChangeOutboxEnqueueError;
use crate::read_model::ReadModelFragmentChangeEnvelopeError;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::ProjectorProcessedEventStoreError;

#[derive(Debug, Error)]
pub enum ProjectorRunnerError {
    #[error("processed event store failed: {0}")]
    ProcessedEventStore(#[from] ProjectorProcessedEventStoreError),

    #[error("read model fragment change outbox enqueue failed: {0}")]
    ReadModelFragmentChangeOutbox(#[from] ReadModelFragmentChangeOutboxEnqueueError),

    #[error(transparent)]
    ReadModelFragmentChangeEnvelope(#[from] ReadModelFragmentChangeEnvelopeError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("projector definition failed")]
    Definition(#[source] Box<dyn std::error::Error + Send + Sync>),
}
