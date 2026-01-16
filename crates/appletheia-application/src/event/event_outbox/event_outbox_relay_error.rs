use thiserror::Error;

use super::{
    EventOutboxError, EventOutboxFetcherError, EventOutboxPublisherError, EventOutboxState,
    EventOutboxWriterError,
};
use crate::unit_of_work::UnitOfWorkError;

#[derive(Debug, Error)]
pub enum EventOutboxRelayError {
    #[error("outbox fetching failed: {0}")]
    Fetcher(#[from] EventOutboxFetcherError),

    #[error("outbox publisher failed: {0}")]
    Publisher(#[from] EventOutboxPublisherError),

    #[error("outbox writer failed: {0}")]
    Writer(#[from] EventOutboxWriterError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("outbox error: {0}")]
    EventOutbox(#[from] EventOutboxError),

    #[error("outbox state must be pending but was {0:?}")]
    NonPendingOutboxState(EventOutboxState),
}
