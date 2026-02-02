use thiserror::Error;

use super::ProjectorRunnerError;
use crate::{ConsumerError, TopicError};

#[derive(Debug, Error)]
pub enum ProjectorWorkerError {
    #[error(transparent)]
    Topic(#[from] TopicError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Runner(#[from] ProjectorRunnerError),
}
