use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a `Team`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TeamStatus {
    Active,
    Removed,
}

impl TeamStatus {
    /// Returns whether the team is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the team is removed.
    pub fn is_removed(&self) -> bool {
        matches!(self, Self::Removed)
    }
}

#[cfg(test)]
mod tests {
    use super::TeamStatus;

    #[test]
    fn active_status_is_active() {
        assert!(TeamStatus::Active.is_active());
        assert!(!TeamStatus::Active.is_removed());
    }

    #[test]
    fn removed_status_is_removed() {
        assert!(!TeamStatus::Removed.is_active());
        assert!(TeamStatus::Removed.is_removed());
    }
}
