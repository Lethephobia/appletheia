use crate::projection::{ReadYourWritesPollInterval, ReadYourWritesTarget, ReadYourWritesTimeout};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum QueryConsistency {
    #[default]
    Eventual,
    ReadYourWrites {
        target: ReadYourWritesTarget,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
    },
}
