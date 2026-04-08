use crate::projection::{ReadYourWritesPollInterval, ReadYourWritesTarget, ReadYourWritesTimeout};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum CommandConsistency {
    #[default]
    Eventual,
    ReadYourWrites {
        target: ReadYourWritesTarget,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
    },
}
