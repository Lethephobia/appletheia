use serde::{Deserialize, Serialize};

/// Describes why a currency name change request was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyNameChangeRejectionReason {
    Removed,
}
