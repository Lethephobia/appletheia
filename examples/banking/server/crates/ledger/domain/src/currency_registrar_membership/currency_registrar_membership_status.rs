use serde::{Deserialize, Serialize};

/// Describes a CurrencyRegistrarMembership lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyRegistrarMembershipStatus {
    Active,
    Removed,
}

impl CurrencyRegistrarMembershipStatus {
    /// Returns whether the membership is active.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}
