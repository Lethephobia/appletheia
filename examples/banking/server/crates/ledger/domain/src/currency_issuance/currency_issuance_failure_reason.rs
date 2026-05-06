use serde::{Deserialize, Serialize};

/// Describes why a requested currency issuance failed after orchestration began.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyIssuanceFailureReason {
    DepositRejected,
    SupplyDecreaseRejected,
}
