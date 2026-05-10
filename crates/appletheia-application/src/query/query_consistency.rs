use crate::projection::{ReadYourWritesPollInterval, ReadYourWritesTimeout};
use crate::request_context::MessageId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum QueryConsistency {
    #[default]
    Eventual,
    ReadYourWrites {
        message_id: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
    },
}
