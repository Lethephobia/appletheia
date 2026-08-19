use thiserror::Error;

use crate::messaging::{ConsumerError, SubscriberError};

use super::ReadModelWatchRegistryError;

/// Reports a fixed-shard invalidation subscriber or dispatch failure.
#[derive(Debug, Error)]
pub enum ReadModelInvalidationWorkerError {
    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Dispatch(#[from] ReadModelWatchRegistryError),
}
