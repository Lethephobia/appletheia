use super::{Outbox, OutboxBatchSize, OutboxFetcherError};

#[allow(async_fn_in_trait)]
pub trait OutboxFetcher {
    async fn fetch(&mut self, limit: OutboxBatchSize) -> Result<Vec<Outbox>, OutboxFetcherError>;
}
