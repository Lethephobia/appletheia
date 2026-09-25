use serde::{Deserialize, Serialize};

/// Describes why a Currency lifecycle command was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyLifecycleRejectionReason {
    AlreadyActive,
    AlreadyInactive,
}
