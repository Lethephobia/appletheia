use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a currency issuance.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyIssuanceStatus {
    Pending,
    Completed,
    Failed,
}
