use thiserror::Error;

use super::SagaRunnerError;
use crate::{ConsumerError, SubscriberError};

#[derive(Debug, Error)]
pub enum SagaCommandFailureWorkerError {
    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Runner(#[from] SagaRunnerError),
}
