use crate::projection::{ReadYourWritesPollInterval, ReadYourWritesTimeout};
use crate::request_context::MessageId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CommandConsistency {
    Eventual,
    ReadYourWrites {
        after: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
    },
}

impl Default for CommandConsistency {
    fn default() -> Self {
        Self::Eventual
    }
}
