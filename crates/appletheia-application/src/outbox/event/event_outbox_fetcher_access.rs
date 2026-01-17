use super::EventOutboxFetcher;

pub trait EventOutboxFetcherAccess {
    type Fetcher: EventOutboxFetcher;

    fn outbox_fetcher(&self) -> &Self::Fetcher;
}
