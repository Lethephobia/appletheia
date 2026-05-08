use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an owned account closure.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureStatus {
    Requested,
    InProgress,
    Completed,
    Failed,
}

impl OwnedAccountClosureStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }
}
