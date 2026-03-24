use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a transfer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}
