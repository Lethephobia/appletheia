use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a `Currency`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyStatus {
    Active,
    Inactive,
    Removed,
}

impl CurrencyStatus {
    /// Returns whether the currency is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the currency is inactive.
    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }

    /// Returns whether the currency is removed.
    pub fn is_removed(&self) -> bool {
        matches!(self, Self::Removed)
    }
}

#[cfg(test)]
mod tests {
    use super::CurrencyStatus;

    #[test]
    fn active_status_is_active() {
        assert!(CurrencyStatus::Active.is_active());
        assert!(!CurrencyStatus::Active.is_inactive());
        assert!(!CurrencyStatus::Active.is_removed());
    }

    #[test]
    fn inactive_status_is_inactive() {
        assert!(!CurrencyStatus::Inactive.is_active());
        assert!(CurrencyStatus::Inactive.is_inactive());
        assert!(!CurrencyStatus::Inactive.is_removed());
    }

    #[test]
    fn removed_status_is_removed() {
        assert!(!CurrencyStatus::Removed.is_active());
        assert!(!CurrencyStatus::Removed.is_inactive());
        assert!(CurrencyStatus::Removed.is_removed());
    }
}
