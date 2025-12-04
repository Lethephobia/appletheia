use super::{Outbox, OutboxPublishResult, OutboxPublisherError};

#[allow(async_fn_in_trait)]
pub trait OutboxPublisher {
    async fn publish_outbox(
        &self,
        outboxes: &[Outbox],
    ) -> Result<Vec<OutboxPublishResult>, OutboxPublisherError>;
}
