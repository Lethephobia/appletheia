use super::CommandOutboxFetcher;

pub trait CommandOutboxFetcherAccess {
    type Fetcher: CommandOutboxFetcher;

    fn outbox_fetcher(&self) -> &Self::Fetcher;
}
