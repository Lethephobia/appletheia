use std::sync::Arc;

use super::OutboxId;

#[derive(Clone)]
pub enum OutboxPublishResult {
    Success {
        outbox_id: OutboxId,
        transport_message_id: Option<String>,
    },
    Failed {
        outbox_id: OutboxId,
        source: Arc<dyn Send + Sync + 'static>,
    },
}
