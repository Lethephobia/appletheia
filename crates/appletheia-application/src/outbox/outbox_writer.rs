use super::{Outbox, OutboxWriterError};
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait OutboxWriter {
    type Uow: UnitOfWork;

    async fn write_outbox(
        &self,
        uow: &mut Self::Uow,
        outboxes: &[Outbox],
    ) -> Result<(), OutboxWriterError>;
}
