use super::{Outbox, OutboxPublisherError};

#[allow(async_fn_in_trait)]
pub trait OutboxPublisher {
    async fn publish_outbox(&self, outbox: &[Outbox]) -> Result<(), OutboxPublisherError>;
}
