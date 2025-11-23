use super::{
    Outbox, OutboxBatchSize, OutboxFetcherError, OutboxLeaseDuration, OutboxRelayInstance,
};

#[allow(async_fn_in_trait)]
pub trait OutboxFetcher {
    async fn fetch_and_acquire_outbox(
        &mut self,
        limit: OutboxBatchSize,
        owner: &OutboxRelayInstance,
        lease_for: OutboxLeaseDuration,
    ) -> Result<Vec<Outbox>, OutboxFetcherError>;
}
