use super::CommandOutboxId;
use crate::outbox::OutboxDispatchError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandOutboxPublishResult {
    Success {
        input_index: usize,
        outbox_id: CommandOutboxId,
        transport_message_id: Option<String>,
    },
    Failed {
        input_index: usize,
        outbox_id: CommandOutboxId,
        cause: OutboxDispatchError,
    },
}
