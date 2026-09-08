use thiserror::Error;

use crate::outbox::command::CommandOutboxEnqueueError;
use crate::unit_of_work::UnitOfWorkError;
use crate::unit_of_work::UnitOfWorkFactoryError;

use super::{
    SagaInstanceStoreError, SagaProcessedCommandFailureStoreError, SagaProcessedEventStoreError,
    SagaRouteError,
};

#[derive(Debug, Error)]
pub enum SagaRunnerError<E: std::error::Error + Send + Sync + 'static> {
    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    Store(#[from] SagaInstanceStoreError),

    #[error(transparent)]
    ProcessedCommandFailureStore(#[from] SagaProcessedCommandFailureStoreError),

    #[error(transparent)]
    ProcessedEventStore(#[from] SagaProcessedEventStoreError),

    #[error(transparent)]
    CommandOutbox(#[from] CommandOutboxEnqueueError),

    #[error(transparent)]
    Route(#[from] SagaRouteError<E>),
}
