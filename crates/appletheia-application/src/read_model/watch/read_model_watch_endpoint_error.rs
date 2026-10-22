use std::error::Error as StdError;

use thiserror::Error;

use crate::{
    messaging::{ConsumerError, ConsumerGroupError, DeliveryError, SubscriberError},
    unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError},
};

use super::{ReadModelWatchEndpointRegistryError, ReadModelWatchLookupError};

#[derive(Debug, Error)]
pub enum ReadModelWatchEndpointError {
    #[error("watch endpoint state mutex is poisoned")]
    StatePoisoned,

    #[error("watch endpoint has already been started")]
    AlreadyStarted,

    #[error("watch endpoint is closed")]
    Closed,

    #[error("watch is already registered locally")]
    WatchAlreadyRegistered,

    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error(transparent)]
    ConsumerGroup(#[from] ConsumerGroupError),

    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Delivery(#[from] DeliveryError),

    #[error(transparent)]
    EndpointRegistry(#[from] ReadModelWatchEndpointRegistryError),

    #[error(transparent)]
    Lookup(#[from] ReadModelWatchLookupError),

    #[error("read model watch endpoint backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
