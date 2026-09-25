use serde::{Deserialize, Serialize};

/// Describes whether a Currency accepts new Accounts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyStatus {
    Defined,
    Active,
    Inactive,
}

impl CurrencyStatus {
    /// Returns whether the currency is active.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}
