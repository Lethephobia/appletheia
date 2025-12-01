use std::sync::Arc;

use super::OutboxId;

#[derive(Clone)]
pub enum OutboxPublishResult {
    Success {
        input_index: usize,
        outbox_id: OutboxId,
        transport_message_id: Option<String>,
    },
    Failed {
        input_index: usize,
        outbox_id: OutboxId,
        source: Arc<dyn Send + Sync + 'static>,
    },
}
