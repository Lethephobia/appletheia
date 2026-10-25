use thiserror::Error;

use crate::messaging::ConsumerGroupError;

use super::ProjectorRunnerError;
use crate::{ConsumerError, DeliveryError, SubscriberError};

#[derive(Debug, Error)]
pub enum ProjectorWorkerError {
    #[error(transparent)]
    ConsumerGroup(#[from] ConsumerGroupError),

    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Delivery(#[from] DeliveryError),

    #[error(transparent)]
    Runner(#[from] ProjectorRunnerError),
}
