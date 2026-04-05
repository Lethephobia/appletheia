use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `OrganizationMembership`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationMembershipStatus {
    Active,
    Inactive,
    Removed,
}

impl OrganizationMembershipStatus {
    /// Returns whether the membership is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the membership is inactive.
    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }

    /// Returns whether the membership is removed.
    pub fn is_removed(&self) -> bool {
        matches!(self, Self::Removed)
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationMembershipStatus;

    #[test]
    fn active_status_is_active() {
        assert!(OrganizationMembershipStatus::Active.is_active());
        assert!(!OrganizationMembershipStatus::Active.is_inactive());
        assert!(!OrganizationMembershipStatus::Active.is_removed());
    }

    #[test]
    fn inactive_status_is_inactive() {
        assert!(!OrganizationMembershipStatus::Inactive.is_active());
        assert!(OrganizationMembershipStatus::Inactive.is_inactive());
        assert!(!OrganizationMembershipStatus::Inactive.is_removed());
    }

    #[test]
    fn removed_status_is_removed() {
        assert!(!OrganizationMembershipStatus::Removed.is_active());
        assert!(!OrganizationMembershipStatus::Removed.is_inactive());
        assert!(OrganizationMembershipStatus::Removed.is_removed());
    }
}
