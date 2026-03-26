use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a `UserRoleAssignment`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRoleAssignmentStatus {
    Assigned,
    Revoked,
}

impl UserRoleAssignmentStatus {
    /// Returns whether the assignment is currently active.
    pub fn is_assigned(&self) -> bool {
        matches!(self, Self::Assigned)
    }

    /// Returns whether the assignment is permanently revoked.
    pub fn is_revoked(&self) -> bool {
        matches!(self, Self::Revoked)
    }
}

#[cfg(test)]
mod tests {
    use super::UserRoleAssignmentStatus;

    #[test]
    fn assigned_status_is_assigned() {
        assert!(UserRoleAssignmentStatus::Assigned.is_assigned());
        assert!(!UserRoleAssignmentStatus::Assigned.is_revoked());
    }

    #[test]
    fn revoked_status_is_revoked() {
        assert!(!UserRoleAssignmentStatus::Revoked.is_assigned());
        assert!(UserRoleAssignmentStatus::Revoked.is_revoked());
    }
}
