use serde::{Deserialize, Serialize};

/// Describes why recording owned account closure progress was rejected.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureRecordRejectionReason {
    AlreadyTerminal,
}
