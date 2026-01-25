use thiserror::Error;

use super::SagaRunnerError;
use crate::{ConsumerBuilderError, ConsumerError};

#[derive(Debug, Error)]
pub enum SagaWorkerError {
    #[error(transparent)]
    ConsumerBuilder(#[from] ConsumerBuilderError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Runner(#[from] SagaRunnerError),
}
