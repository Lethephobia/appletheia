use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `OrganizationMembership`.
///
/// Deletion is terminal. Rejoining an organization after deletion creates a new
/// membership aggregate with a new `OrganizationMembershipId` rather than
/// reviving the deleted one.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipStatus {
    Active,
    Deleted,
}

impl OrganizationMembershipStatus {
    /// Returns whether the membership is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the membership is deleted.
    pub fn is_deleted(&self) -> bool {
        matches!(self, Self::Deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationMembershipStatus;

    #[test]
    fn active_status_is_active() {
        assert!(OrganizationMembershipStatus::Active.is_active());
        assert!(!OrganizationMembershipStatus::Active.is_deleted());
    }

    #[test]
    fn deleted_status_is_deleted() {
        assert!(!OrganizationMembershipStatus::Deleted.is_active());
        assert!(OrganizationMembershipStatus::Deleted.is_deleted());
    }
}
