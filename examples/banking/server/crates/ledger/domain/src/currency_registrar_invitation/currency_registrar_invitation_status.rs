use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `CurrencyRegistrarInvitation`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarInvitationStatus {
    Pending,
    Accepted,
    Declined,
    Canceled,
    Rejected,
}

impl CurrencyRegistrarInvitationStatus {
    /// Returns whether the invitation is pending.
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns whether the invitation is accepted.
    pub fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }

    /// Returns whether the invitation is declined.
    pub fn is_declined(&self) -> bool {
        matches!(self, Self::Declined)
    }

    /// Returns whether the invitation is canceled.
    pub fn is_canceled(&self) -> bool {
        matches!(self, Self::Canceled)
    }

    /// Returns whether the invitation is rejected.
    pub fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected)
    }
}
