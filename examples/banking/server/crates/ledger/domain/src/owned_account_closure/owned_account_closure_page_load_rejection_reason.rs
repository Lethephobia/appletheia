use serde::{Deserialize, Serialize};

/// Describes why loading an owned account closure page was rejected.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosurePageLoadRejectionReason {
    AlreadyTerminal,
}
