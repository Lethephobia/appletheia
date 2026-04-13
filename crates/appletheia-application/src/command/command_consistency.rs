use crate::projection::{ReadYourWritesPollInterval, ReadYourWritesTarget, ReadYourWritesTimeout};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CommandConsistency {
    #[default]
    Eventual,
    ReadYourWrites {
        target: ReadYourWritesTarget,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
    },
}
