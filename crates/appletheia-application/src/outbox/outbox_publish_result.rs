use super::OutboxDispatchError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutboxPublishResult<Id> {
    Success {
        input_index: usize,
        outbox_id: Id,
        transport_message_id: Option<String>,
    },
    Failed {
        input_index: usize,
        outbox_id: Id,
        cause: OutboxDispatchError,
    },
}
