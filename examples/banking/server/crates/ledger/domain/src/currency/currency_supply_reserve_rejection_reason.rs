use serde::{Deserialize, Serialize};

/// Describes why a reserve-supply request was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencySupplyReserveRejectionReason {
    ProvisioningPending,
    Inactive,
    Removed,
    SupplyOverflow,
}
