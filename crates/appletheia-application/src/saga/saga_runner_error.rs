use thiserror::Error;

use crate::event::EventEnvelopeError;
use crate::outbox::command::{CommandEnvelopeError, CommandOutboxEnqueueError};
use crate::unit_of_work::UnitOfWorkError;
use crate::unit_of_work::UnitOfWorkFactoryError;

use super::{SagaProcessedEventStoreError, SagaRunStoreError};

#[derive(Debug, Error)]
pub enum SagaRunnerError {
    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    RunStore(#[from] SagaRunStoreError),

    #[error(transparent)]
    ProcessedEventStore(#[from] SagaProcessedEventStoreError),

    #[error(transparent)]
    CommandOutbox(#[from] CommandOutboxEnqueueError),

    #[error(transparent)]
    CommandEnvelope(#[from] CommandEnvelopeError),

    #[error("event envelope error: {0}")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("saga handler error")]
    Handler(#[source] Box<dyn std::error::Error + Send + Sync>),
}
