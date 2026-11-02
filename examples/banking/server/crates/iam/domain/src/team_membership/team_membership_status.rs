use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TeamMembershipStatus {
    Active,
    Deleted,
}

impl TeamMembershipStatus {
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
    use super::TeamMembershipStatus;

    #[test]
    fn active_status_is_active() {
        assert!(TeamMembershipStatus::Active.is_active());
        assert!(!TeamMembershipStatus::Active.is_deleted());
    }

    #[test]
    fn deleted_status_is_deleted() {
        assert!(!TeamMembershipStatus::Deleted.is_active());
        assert!(TeamMembershipStatus::Deleted.is_deleted());
    }
}
