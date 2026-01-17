use super::EventOutbox;
use crate::outbox::OutboxWriterError;
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait EventOutboxWriter {
    type Uow: UnitOfWork;

    async fn write_outbox(
        &self,
        uow: &mut Self::Uow,
        outboxes: &[EventOutbox],
    ) -> Result<(), OutboxWriterError>;
}
