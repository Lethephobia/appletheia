use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a `User`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Inactive,
    Removed,
}

impl UserStatus {
    /// Returns whether the user is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the user is inactive.
    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }

    /// Returns whether the user is removed.
    pub fn is_removed(&self) -> bool {
        matches!(self, Self::Removed)
    }
}

#[cfg(test)]
mod tests {
    use super::UserStatus;

    #[test]
    fn active_status_is_active() {
        assert!(UserStatus::Active.is_active());
        assert!(!UserStatus::Active.is_inactive());
        assert!(!UserStatus::Active.is_removed());
    }

    #[test]
    fn inactive_status_is_inactive() {
        assert!(!UserStatus::Inactive.is_active());
        assert!(UserStatus::Inactive.is_inactive());
        assert!(!UserStatus::Inactive.is_removed());
    }

    #[test]
    fn removed_status_is_removed() {
        assert!(!UserStatus::Removed.is_active());
        assert!(!UserStatus::Removed.is_inactive());
        assert!(UserStatus::Removed.is_removed());
    }
}
