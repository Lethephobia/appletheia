use super::{Outbox, OutboxBatchSize, OutboxFetcherError};
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait OutboxFetcher {
    type Uow: UnitOfWork;

    async fn fetch(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<Outbox>, OutboxFetcherError>;
}
