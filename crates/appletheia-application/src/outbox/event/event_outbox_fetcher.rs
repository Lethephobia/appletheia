use super::EventOutbox;
use crate::outbox::{OutboxBatchSize, OutboxFetcherError};
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait EventOutboxFetcher {
    type Uow: UnitOfWork;

    async fn fetch(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<EventOutbox>, OutboxFetcherError>;
}
