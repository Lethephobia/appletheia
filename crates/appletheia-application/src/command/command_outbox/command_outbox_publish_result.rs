use super::{CommandOutboxDispatchError, CommandOutboxId};

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
        cause: CommandOutboxDispatchError,
    },
}
