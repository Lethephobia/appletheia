use std::error::Error;

use thiserror::Error;

use super::SagaRunnerError;

#[derive(Debug, Error)]
pub enum SagaWorkerError {
    #[error("saga consumer next error")]
    ConsumerNext(#[source] Box<dyn Error + Send + Sync>),

    #[error("saga consumer ack error")]
    ConsumerAck(#[source] Box<dyn Error + Send + Sync>),

    #[error("saga consumer nack error")]
    ConsumerNack(#[source] Box<dyn Error + Send + Sync>),

    #[error(transparent)]
    Runner(#[from] SagaRunnerError),
}

