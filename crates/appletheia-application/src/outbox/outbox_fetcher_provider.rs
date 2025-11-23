use super::OutboxFetcher;

pub trait OutboxFetcherProvider {
    type OutboxFetcher<'c>: OutboxFetcher
    where
        Self: 'c;

    fn outbox_fetcher(&mut self) -> Self::OutboxFetcher<'_>;
}
