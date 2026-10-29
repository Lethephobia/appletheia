use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `Organization`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationStatus {
    Active,
    Deleted,
}

impl OrganizationStatus {
    /// Returns whether the organization is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the organization is deleted.
    pub fn is_deleted(&self) -> bool {
        matches!(self, Self::Deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationStatus;

    #[test]
    fn active_status_is_active() {
        assert!(OrganizationStatus::Active.is_active());
        assert!(!OrganizationStatus::Active.is_deleted());
    }

    #[test]
    fn deleted_status_is_deleted() {
        assert!(!OrganizationStatus::Deleted.is_active());
        assert!(OrganizationStatus::Deleted.is_deleted());
    }
}
