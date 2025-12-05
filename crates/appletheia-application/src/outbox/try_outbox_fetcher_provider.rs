use std::error::Error;

use super::OutboxFetcher;

pub trait TryOutboxFetcherProvider {
    type Error: Error + Send + Sync + 'static;
    type OutboxFetcher<'c>: OutboxFetcher
    where
        Self: 'c;

    fn try_outbox_fetcher(&mut self) -> Result<Self::OutboxFetcher<'_>, Self::Error>;
}
