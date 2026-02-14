use crate::request_context::MessageId;

use super::{ReadYourWritesPollInterval, ReadYourWritesTimeout};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum QueryConsistency {
    Eventual,
    ReadYourWrites {
        after: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
    },
}

impl Default for QueryConsistency {
    fn default() -> Self {
        Self::Eventual
    }
}
