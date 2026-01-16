use super::{EventOutbox, EventOutboxPublishResult, EventOutboxPublisherError};

#[allow(async_fn_in_trait)]
pub trait EventOutboxPublisher {
    async fn publish_outbox(
        &self,
        outboxes: &[EventOutbox],
    ) -> Result<Vec<EventOutboxPublishResult>, EventOutboxPublisherError>;
}
