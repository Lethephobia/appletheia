use thiserror::Error;

use crate::messaging::{
    ConsumerError, DeliveryError, PublishDispatchError, PublisherError, SubscriberError,
};
use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::ReadModelWatchLookupError;

#[derive(Debug, Error)]
pub enum ReadModelWatchNotificationWorkerError {
    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error(transparent)]
    Lookup(#[from] ReadModelWatchLookupError),

    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Delivery(#[from] DeliveryError),

    #[error(transparent)]
    Publisher(#[from] PublisherError),

    #[error("notification publication failed: {failures:?}")]
    PublishFailed {
        failures: Vec<(usize, PublishDispatchError)>,
    },
}
