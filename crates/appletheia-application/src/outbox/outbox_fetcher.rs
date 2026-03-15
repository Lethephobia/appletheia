use super::{OutboxBatchSize, OutboxFetcherError};
use crate::unit_of_work::UnitOfWork;

use super::Outbox;

#[allow(async_fn_in_trait)]
pub trait OutboxFetcher: Send + Sync {
    type Uow: UnitOfWork;
    type Outbox: Outbox;

    async fn fetch_pending(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<Self::Outbox>, OutboxFetcherError>;

    async fn fetch_dead_lettered(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<Self::Outbox>, OutboxFetcherError>;
}
