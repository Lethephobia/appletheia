use crate::request_context::MessageId;

use crate::projection::{ReadYourWritesPollInterval, ReadYourWritesTimeout};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum QueryConsistency {
    #[default]
    Eventual,
    ReadYourWrites {
        after: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
    },
}
