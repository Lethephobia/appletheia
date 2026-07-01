use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a `Currency`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyStatus {
    Provisioning,
    Active,
    Inactive,
    ProvisioningFailed,
    Removed,
}

impl CurrencyStatus {
    /// Returns whether the currency is being provisioned.
    pub fn is_provisioning(&self) -> bool {
        matches!(self, Self::Provisioning)
    }

    /// Returns whether the currency is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the currency is inactive.
    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }

    /// Returns whether the currency provisioning has failed.
    pub fn is_provisioning_failed(&self) -> bool {
        matches!(self, Self::ProvisioningFailed)
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
    fn provisioning_status_is_provisioning() {
        assert!(CurrencyStatus::Provisioning.is_provisioning());
        assert!(!CurrencyStatus::Provisioning.is_active());
        assert!(!CurrencyStatus::Provisioning.is_inactive());
        assert!(!CurrencyStatus::Provisioning.is_provisioning_failed());
        assert!(!CurrencyStatus::Provisioning.is_removed());
    }

    #[test]
    fn active_status_is_active() {
        assert!(!CurrencyStatus::Active.is_provisioning());
        assert!(CurrencyStatus::Active.is_active());
        assert!(!CurrencyStatus::Active.is_inactive());
        assert!(!CurrencyStatus::Active.is_provisioning_failed());
        assert!(!CurrencyStatus::Active.is_removed());
    }

    #[test]
    fn inactive_status_is_inactive() {
        assert!(!CurrencyStatus::Inactive.is_provisioning());
        assert!(!CurrencyStatus::Inactive.is_active());
        assert!(CurrencyStatus::Inactive.is_inactive());
        assert!(!CurrencyStatus::Inactive.is_provisioning_failed());
        assert!(!CurrencyStatus::Inactive.is_removed());
    }

    #[test]
    fn provisioning_failed_status_is_provisioning_failed() {
        assert!(!CurrencyStatus::ProvisioningFailed.is_provisioning());
        assert!(!CurrencyStatus::ProvisioningFailed.is_active());
        assert!(!CurrencyStatus::ProvisioningFailed.is_inactive());
        assert!(CurrencyStatus::ProvisioningFailed.is_provisioning_failed());
        assert!(!CurrencyStatus::ProvisioningFailed.is_removed());
    }

    #[test]
    fn removed_status_is_removed() {
        assert!(!CurrencyStatus::Removed.is_provisioning());
        assert!(!CurrencyStatus::Removed.is_active());
        assert!(!CurrencyStatus::Removed.is_inactive());
        assert!(!CurrencyStatus::Removed.is_provisioning_failed());
        assert!(CurrencyStatus::Removed.is_removed());
    }
}
