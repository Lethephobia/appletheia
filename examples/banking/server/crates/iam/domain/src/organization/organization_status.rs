use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `Organization`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationStatus {
    Active,
    Removed,
}

impl OrganizationStatus {
    /// Returns whether the organization is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the organization is removed.
    pub fn is_removed(&self) -> bool {
        matches!(self, Self::Removed)
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationStatus;

    #[test]
    fn active_status_is_active() {
        assert!(OrganizationStatus::Active.is_active());
        assert!(!OrganizationStatus::Active.is_removed());
    }

    #[test]
    fn removed_status_is_removed() {
        assert!(!OrganizationStatus::Removed.is_active());
        assert!(OrganizationStatus::Removed.is_removed());
    }
}
