use thiserror::Error;

use crate::outbox::command::CommandOutboxEnqueueError;
use crate::unit_of_work::UnitOfWorkError;
use crate::unit_of_work::UnitOfWorkFactoryError;

use super::{SagaProcessedEventStoreError, SagaStoreError};

#[derive(Debug, Error)]
pub enum SagaRunnerError {
    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    Store(#[from] SagaStoreError),

    #[error(transparent)]
    ProcessedEventStore(#[from] SagaProcessedEventStoreError),

    #[error(transparent)]
    CommandOutbox(#[from] CommandOutboxEnqueueError),

    #[error("saga definition error")]
    Definition(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("terminal outcome requires non-empty saga state")]
    TerminalOutcomeRequiresState,
}
