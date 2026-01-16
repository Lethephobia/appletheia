use super::{CommandOutbox, CommandOutboxPublishResult, CommandOutboxPublisherError};

#[allow(async_fn_in_trait)]
pub trait CommandOutboxPublisher {
    async fn publish_outbox(
        &self,
        outboxes: &[CommandOutbox],
    ) -> Result<Vec<CommandOutboxPublishResult>, CommandOutboxPublisherError>;
}
