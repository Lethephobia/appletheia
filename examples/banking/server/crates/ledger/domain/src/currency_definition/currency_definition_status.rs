use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a `CurrencyDefinition`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyDefinitionStatus {
    Active,
    Inactive,
    Removed,
}

impl CurrencyDefinitionStatus {
    /// Returns whether the currency definition is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the currency definition is inactive.
    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }

    /// Returns whether the currency definition is removed.
    pub fn is_removed(&self) -> bool {
        matches!(self, Self::Removed)
    }
}

#[cfg(test)]
mod tests {
    use super::CurrencyDefinitionStatus;

    #[test]
    fn active_status_is_active() {
        assert!(CurrencyDefinitionStatus::Active.is_active());
        assert!(!CurrencyDefinitionStatus::Active.is_inactive());
        assert!(!CurrencyDefinitionStatus::Active.is_removed());
    }

    #[test]
    fn inactive_status_is_inactive() {
        assert!(!CurrencyDefinitionStatus::Inactive.is_active());
        assert!(CurrencyDefinitionStatus::Inactive.is_inactive());
        assert!(!CurrencyDefinitionStatus::Inactive.is_removed());
    }

    #[test]
    fn removed_status_is_removed() {
        assert!(!CurrencyDefinitionStatus::Removed.is_active());
        assert!(!CurrencyDefinitionStatus::Removed.is_inactive());
        assert!(CurrencyDefinitionStatus::Removed.is_removed());
    }
}
