use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TeamMembershipStatus {
    Active,
    Removed,
}

impl TeamMembershipStatus {
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
    use super::TeamMembershipStatus;

    #[test]
    fn active_status_is_active() {
        assert!(TeamMembershipStatus::Active.is_active());
        assert!(!TeamMembershipStatus::Active.is_removed());
    }

    #[test]
    fn removed_status_is_removed() {
        assert!(!TeamMembershipStatus::Removed.is_active());
        assert!(TeamMembershipStatus::Removed.is_removed());
    }
}
