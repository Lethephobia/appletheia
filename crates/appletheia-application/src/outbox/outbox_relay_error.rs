use thiserror::Error;

use crate::outbox::{
    OutboxError, OutboxFetcherError, OutboxPublisherError, OutboxState, OutboxWriterError,
};
use crate::unit_of_work::UnitOfWorkError;

#[derive(Debug, Error)]
pub enum OutboxRelayError {
    #[error("outbox fetching failed: {0}")]
    Fetcher(#[from] OutboxFetcherError),

    #[error("outbox publisher failed: {0}")]
    Publisher(#[from] OutboxPublisherError),

    #[error("outbox writer failed: {0}")]
    Writer(#[from] OutboxWriterError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("outbox error: {0}")]
    Outbox(#[from] OutboxError),

    #[error("outbox state must be pending but was {0:?}")]
    NonPendingOutboxState(OutboxState),
}
