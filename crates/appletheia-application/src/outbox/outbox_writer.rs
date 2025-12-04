use super::{Outbox, OutboxWriterError};

#[allow(async_fn_in_trait)]
pub trait OutboxWriter {
    async fn write_outbox(&mut self, outboxes: &[Outbox]) -> Result<(), OutboxWriterError>;
}
