use serde::{Deserialize, Serialize};

/// Describes why a funds reservation request was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum AccountFundsReserveRejectionReason {
    Frozen,
    Closed,
    InsufficientAvailableBalance,
}
