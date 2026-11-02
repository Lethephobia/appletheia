use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a `User`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Inactive,
    Deleted,
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

    /// Returns whether the user is deleted.
    pub fn is_deleted(&self) -> bool {
        matches!(self, Self::Deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::UserStatus;

    #[test]
    fn active_status_is_active() {
        assert!(UserStatus::Active.is_active());
        assert!(!UserStatus::Active.is_inactive());
        assert!(!UserStatus::Active.is_deleted());
    }

    #[test]
    fn inactive_status_is_inactive() {
        assert!(!UserStatus::Inactive.is_active());
        assert!(UserStatus::Inactive.is_inactive());
        assert!(!UserStatus::Inactive.is_deleted());
    }

    #[test]
    fn deleted_status_is_deleted() {
        assert!(!UserStatus::Deleted.is_active());
        assert!(!UserStatus::Deleted.is_inactive());
        assert!(UserStatus::Deleted.is_deleted());
    }
}
