use thiserror::Error;

use crate::messaging::ConsumerError;
use crate::messaging::SubscriberError;

use super::CommandEnvelopeError;
use super::CommandExecutionStoreError;
use crate::outbox::command_failure::CommandFailureOutboxEnqueueError;
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

#[derive(Debug, Error)]
pub enum CommandWorkerError {
    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    CommandEnvelope(#[from] CommandEnvelopeError),

    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    ExecutionStore(#[from] CommandExecutionStoreError),

    #[error(transparent)]
    FailureOutbox(#[from] CommandFailureOutboxEnqueueError),
}
