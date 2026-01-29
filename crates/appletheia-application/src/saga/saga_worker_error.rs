use thiserror::Error;

use super::SagaRunnerError;
use crate::{ConsumerError, TopicError};

#[derive(Debug, Error)]
pub enum SagaWorkerError {
    #[error(transparent)]
    Topic(#[from] TopicError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Runner(#[from] SagaRunnerError),
}
