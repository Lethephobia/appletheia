use super::{OutboxDispatchError, OutboxId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutboxPublishResult {
    Success {
        input_index: usize,
        outbox_id: OutboxId,
        transport_message_id: Option<String>,
    },
    Failed {
        input_index: usize,
        outbox_id: OutboxId,
        cause: OutboxDispatchError,
    },
}
