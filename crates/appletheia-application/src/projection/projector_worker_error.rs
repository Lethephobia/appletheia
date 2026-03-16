use thiserror::Error;

use super::ProjectorRunnerError;
use crate::{ConsumerError, SubscriberError};

#[derive(Debug, Error)]
pub enum ProjectorWorkerError {
    #[error(transparent)]
    Subscriber(#[from] SubscriberError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Runner(#[from] ProjectorRunnerError),
}
