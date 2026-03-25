use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `Account`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

impl AccountStatus {
    /// Returns whether the account accepts normal operations.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the account is temporarily unavailable.
    pub fn is_frozen(&self) -> bool {
        matches!(self, Self::Frozen)
    }

    /// Returns whether the account is permanently closed.
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }
}

#[cfg(test)]
mod tests {
    use super::AccountStatus;

    #[test]
    fn active_status_is_active() {
        assert!(AccountStatus::Active.is_active());
        assert!(!AccountStatus::Active.is_frozen());
        assert!(!AccountStatus::Active.is_closed());
    }

    #[test]
    fn frozen_status_is_frozen() {
        assert!(!AccountStatus::Frozen.is_active());
        assert!(AccountStatus::Frozen.is_frozen());
        assert!(!AccountStatus::Frozen.is_closed());
    }

    #[test]
    fn closed_status_is_closed() {
        assert!(!AccountStatus::Closed.is_active());
        assert!(!AccountStatus::Closed.is_frozen());
        assert!(AccountStatus::Closed.is_closed());
    }
}
