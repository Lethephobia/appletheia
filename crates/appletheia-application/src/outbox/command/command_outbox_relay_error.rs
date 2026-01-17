use thiserror::Error;

use crate::outbox::{
    OutboxError, OutboxFetcherError, OutboxPublisherError, OutboxState, OutboxWriterError,
};
use crate::unit_of_work::UnitOfWorkError;

#[derive(Debug, Error)]
pub enum CommandOutboxRelayError {
    #[error("command outbox fetching failed: {0}")]
    Fetcher(#[from] OutboxFetcherError),

    #[error("command outbox publisher failed: {0}")]
    Publisher(#[from] OutboxPublisherError),

    #[error("command outbox writer failed: {0}")]
    Writer(#[from] OutboxWriterError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("command outbox error: {0}")]
    CommandOutbox(#[from] OutboxError),

    #[error("command outbox state must be pending but was {0:?}")]
    NonPendingOutboxState(OutboxState),
}
