use super::{EventOutboxDispatchError, EventOutboxId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventOutboxPublishResult {
    Success {
        input_index: usize,
        outbox_id: EventOutboxId,
        transport_message_id: Option<String>,
    },
    Failed {
        input_index: usize,
        outbox_id: EventOutboxId,
        cause: EventOutboxDispatchError,
    },
}
