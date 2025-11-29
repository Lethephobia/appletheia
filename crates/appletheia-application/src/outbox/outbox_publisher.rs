use super::{Outbox, OutboxPublisherError};

#[allow(async_fn_in_trait)]
pub trait OutboxPublisher {
    async fn publish(&self, outbox: Outbox) -> Result<(), OutboxPublisherError>;
}
