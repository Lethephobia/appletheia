use super::PublishDispatchError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublishResult {
    Success {
        input_index: usize,
        transport_message_id: Option<String>,
    },
    Failed {
        input_index: usize,
        cause: PublishDispatchError,
    },
}
