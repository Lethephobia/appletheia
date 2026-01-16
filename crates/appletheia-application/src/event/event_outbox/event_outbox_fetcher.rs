use super::{EventOutbox, EventOutboxBatchSize, EventOutboxFetcherError};
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait EventOutboxFetcher {
    type Uow: UnitOfWork;

    async fn fetch(
        &self,
        uow: &mut Self::Uow,
        limit: EventOutboxBatchSize,
    ) -> Result<Vec<EventOutbox>, EventOutboxFetcherError>;
}
