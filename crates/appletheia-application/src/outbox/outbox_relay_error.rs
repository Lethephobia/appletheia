use thiserror::Error;

use super::{OutboxFetcherError, OutboxPublisherError};

#[derive(Debug, Error)]
pub enum OutboxRelayError {
    #[error("outbox fetching failed: {0}")]
    Fetcher(#[from] OutboxFetcherError),

    #[error("outbox publisher failed: {0}")]
    Publisher(#[from] OutboxPublisherError),
}
