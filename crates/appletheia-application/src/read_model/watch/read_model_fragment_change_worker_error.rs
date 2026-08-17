use thiserror::Error;

use crate::messaging::{ConsumerError, SubscriberError};

use super::ReadModelWatchFragmentDispatcherError;

/// Reports a fixed-shard subscriber or session fanout failure.
#[derive(Debug, Error)]
pub enum ReadModelFragmentChangeWorkerError {
    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Dispatch(#[from] ReadModelWatchFragmentDispatcherError),
}
