use super::{OutboxBatchSize, OutboxFetcherError};
use crate::unit_of_work::UnitOfWork;

use super::Outbox;

#[allow(async_fn_in_trait)]
pub trait OutboxFetcher {
    type Uow: UnitOfWork;
    type Outbox: Outbox;

    async fn fetch(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<Self::Outbox>, OutboxFetcherError>;
}
