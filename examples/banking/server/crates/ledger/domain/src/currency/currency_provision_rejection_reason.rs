use serde::{Deserialize, Serialize};

/// Describes why currency provisioning was rejected.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyProvisionRejectionReason {
    AlreadyProvisioned,
    Removed,
}
