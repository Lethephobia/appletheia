use thiserror::Error;

use super::{SagaError, SagaRunnerError};
use crate::{ConsumerError, DeliveryError, SubscriberError};

#[derive(Debug, Error)]
pub enum SagaEventWorkerError<E: std::error::Error + Send + Sync + 'static> {
    #[error(transparent)]
    Saga(#[from] SagaError),

    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Delivery(#[from] DeliveryError),

    #[error(transparent)]
    Runner(#[from] SagaRunnerError<E>),
}
