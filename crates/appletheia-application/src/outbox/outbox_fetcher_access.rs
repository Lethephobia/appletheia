use super::OutboxFetcher;

pub trait OutboxFetcherAccess {
    type Fetcher: OutboxFetcher;

    fn outbox_fetcher(&self) -> &Self::Fetcher;
}
