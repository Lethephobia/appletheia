use super::OutboxWriterError;
use crate::unit_of_work::UnitOfWork;

use super::Outbox;

#[allow(async_fn_in_trait)]
pub trait OutboxWriter {
    type Uow: UnitOfWork;
    type Outbox: Outbox;

    async fn write_outbox(
        &self,
        uow: &mut Self::Uow,
        outboxes: &[Self::Outbox],
    ) -> Result<(), OutboxWriterError>;
}
