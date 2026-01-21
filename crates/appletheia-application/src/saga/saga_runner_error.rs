use thiserror::Error;

use crate::outbox::OrderingKeyError;
use crate::outbox::command::CommandOutboxEnqueueError;
use crate::unit_of_work::UnitOfWorkError;

use super::{SagaProcessedEventStoreError, SagaStoreError};

#[derive(Debug, Error)]
pub enum SagaRunnerError {
    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),
    #[error(transparent)]
    Store(#[from] SagaStoreError),
    #[error(transparent)]
    ProcessedEventStore(#[from] SagaProcessedEventStoreError),
    #[error(transparent)]
    CommandOutbox(#[from] CommandOutboxEnqueueError),
    #[error(transparent)]
    OrderingKey(#[from] OrderingKeyError),
    #[error("terminal outcome requires non-empty saga state")]
    TerminalOutcomeRequiresState,
}
