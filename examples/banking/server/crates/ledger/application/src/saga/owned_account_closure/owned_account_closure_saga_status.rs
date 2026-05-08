use serde::{Deserialize, Serialize};

/// Describes progress for the owned account closure saga.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureSagaStatus {
    Requested,
    PageLoadRequested,
    AccountCloseRequested,
    Completed,
    Failed,
}
