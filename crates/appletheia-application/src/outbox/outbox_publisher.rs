use super::{OutboxPublishResult, OutboxPublisherError};

use super::Outbox;

#[allow(async_fn_in_trait)]
pub trait OutboxPublisher {
    type Outbox: Outbox;

    async fn publish_outbox(
        &self,
        outboxes: &[Self::Outbox],
    ) -> Result<Vec<OutboxPublishResult<<Self::Outbox as Outbox>::Id>>, OutboxPublisherError>;
}
