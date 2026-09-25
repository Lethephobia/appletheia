use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `OrganizationMembership`.
///
/// Removal is terminal. Rejoining an organization after removal creates a new
/// membership aggregate with a new `OrganizationMembershipId` rather than
/// reviving the removed one.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipStatus {
    Active,
    Removed,
}

impl OrganizationMembershipStatus {
    /// Returns whether the membership is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
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
        assert!(!OrganizationMembershipStatus::Active.is_removed());
    }

    #[test]
    fn removed_status_is_removed() {
        assert!(!OrganizationMembershipStatus::Removed.is_active());
        assert!(OrganizationMembershipStatus::Removed.is_removed());
    }
}
