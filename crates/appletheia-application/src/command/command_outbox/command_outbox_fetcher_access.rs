use super::CommandOutboxFetcher;

pub trait CommandOutboxFetcherAccess {
    type Fetcher: CommandOutboxFetcher;

    fn command_outbox_fetcher(&self) -> &Self::Fetcher;
}
