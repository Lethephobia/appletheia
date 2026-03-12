use crate::projection::{ReadYourWritesPollInterval, ReadYourWritesTimeout};
use crate::request_context::MessageId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum CommandConsistency {
    #[default]
    Eventual,
    ReadYourWrites {
        after: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
    },
}
