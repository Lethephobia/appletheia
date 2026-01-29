use thiserror::Error;

use crate::massaging::PublisherError;
use crate::outbox::{OutboxError, OutboxFetcherError, OutboxState, OutboxWriterError};
use crate::unit_of_work::UnitOfWorkError;
use crate::unit_of_work::UnitOfWorkFactoryError;

#[derive(Debug, Error)]
pub enum OutboxRelayError {
    #[error("outbox fetching failed: {0}")]
    Fetcher(#[from] OutboxFetcherError),

    #[error("publisher failed: {0}")]
    Publisher(#[from] PublisherError),

    #[error("outbox writer failed: {0}")]
    Writer(#[from] OutboxWriterError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("unit of work factory error: {0}")]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("outbox error: {0}")]
    Outbox(#[from] OutboxError),

    #[error("outbox state must be pending but was {0:?}")]
    NonPendingOutboxState(OutboxState),
}
