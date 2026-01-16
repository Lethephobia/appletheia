use super::{CommandOutbox, CommandOutboxBatchSize, CommandOutboxFetcherError};
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait CommandOutboxFetcher {
    type Uow: UnitOfWork;

    async fn fetch(
        &self,
        uow: &mut Self::Uow,
        limit: CommandOutboxBatchSize,
    ) -> Result<Vec<CommandOutbox>, CommandOutboxFetcherError>;
}
