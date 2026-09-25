use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `CurrencyRegistrarJoinRequest`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarJoinRequestStatus {
    Pending,
    Approved,
    Rejected,
    Canceled,
}

impl CurrencyRegistrarJoinRequestStatus {
    /// Returns whether the join request is pending.
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns whether the join request is approved.
    pub fn is_approved(&self) -> bool {
        matches!(self, Self::Approved)
    }

    /// Returns whether the join request is rejected.
    pub fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Returns whether the join request is canceled.
    pub fn is_canceled(&self) -> bool {
        matches!(self, Self::Canceled)
    }
}
