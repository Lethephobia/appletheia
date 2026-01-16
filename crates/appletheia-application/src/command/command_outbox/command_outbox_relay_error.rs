use thiserror::Error;

use super::{
    CommandOutboxError, CommandOutboxFetcherError, CommandOutboxPublisherError, CommandOutboxState,
    CommandOutboxWriterError,
};
use crate::unit_of_work::UnitOfWorkError;

#[derive(Debug, Error)]
pub enum CommandOutboxRelayError {
    #[error("command outbox fetching failed: {0}")]
    Fetcher(#[from] CommandOutboxFetcherError),

    #[error("command outbox publisher failed: {0}")]
    Publisher(#[from] CommandOutboxPublisherError),

    #[error("command outbox writer failed: {0}")]
    Writer(#[from] CommandOutboxWriterError),

    #[error("unit of work error: {0}")]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error("command outbox error: {0}")]
    CommandOutbox(#[from] CommandOutboxError),

    #[error("command outbox state must be pending but was {0:?}")]
    NonPendingOutboxState(CommandOutboxState),
}
