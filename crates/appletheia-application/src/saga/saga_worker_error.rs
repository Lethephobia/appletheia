use thiserror::Error;

use super::SagaRunnerError;
use crate::{ConsumerError, ConsumerFactoryError};

#[derive(Debug, Error)]
pub enum SagaWorkerError {
    #[error(transparent)]
    ConsumerFactory(#[from] ConsumerFactoryError),

    #[error(transparent)]
    Consumer(#[from] ConsumerError),

    #[error(transparent)]
    Runner(#[from] SagaRunnerError),
}
