use thiserror::Error;

use crate::outbox::OrderingKeyError;
use crate::outbox::command::CommandOutboxEnqueueError;
use crate::unit_of_work::UnitOfWorkError;

use super::SagaStoreError;

#[derive(Debug, Error)]
pub enum SagaRunnerError {
    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),
    #[error(transparent)]
    Store(#[from] SagaStoreError),
    #[error(transparent)]
    CommandOutbox(#[from] CommandOutboxEnqueueError),
    #[error(transparent)]
    OrderingKey(#[from] OrderingKeyError),
    #[error("failed to deserialize saga state")]
    StateDeserialize(#[source] serde_json::Error),
    #[error("failed to serialize saga state")]
    StateSerialize(#[source] serde_json::Error),
}
