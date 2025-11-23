use thiserror::Error;

use super::OutboxFetcherError;

#[derive(Debug, Error)]
pub enum OutboxRelayError {
    #[error("outbox fetching failed: {0}")]
    Fetcher(#[from] OutboxFetcherError),
}
