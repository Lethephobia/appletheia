use thiserror::Error;

use super::{OutboxFetcherError, OutboxPublisherError, OutboxWriterError};

#[derive(Debug, Error)]
pub enum OutboxRelayError {
    #[error("outbox fetching failed: {0}")]
    Fetcher(#[from] OutboxFetcherError),

    #[error("outbox publisher failed: {0}")]
    Publisher(#[from] OutboxPublisherError),

    #[error("outbox writer failed: {0}")]
    Writer(#[from] OutboxWriterError),
}
