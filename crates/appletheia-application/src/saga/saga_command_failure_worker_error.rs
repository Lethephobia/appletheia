use thiserror::Error;

use super::{SagaError, SagaRunnerError};
use crate::{ConsumerError, SubscriberError};

#[derive(Debug, Error)]
pub enum SagaCommandFailureWorkerError<E: std::error::Error + Send + Sync + 'static> {
    #[error(transparent)]
    Saga(#[from] SagaError),

    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Runner(#[from] SagaRunnerError<E>),
}
