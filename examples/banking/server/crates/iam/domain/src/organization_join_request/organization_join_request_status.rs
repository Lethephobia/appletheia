use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `OrganizationJoinRequest`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationJoinRequestStatus {
    Pending,
    Approved,
    Rejected,
    Canceled,
}

impl OrganizationJoinRequestStatus {
    /// Returns whether the join request is pending.
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns whether the join request is approved.
    pub fn is_approved(&self) -> bool {
        matches!(self, Self::Approved)
    }

    /// Returns whether the join request is rejected.
    pub fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Returns whether the join request is canceled.
    pub fn is_canceled(&self) -> bool {
        matches!(self, Self::Canceled)
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationJoinRequestStatus;

    #[test]
    fn pending_status_is_pending() {
        assert!(OrganizationJoinRequestStatus::Pending.is_pending());
        assert!(!OrganizationJoinRequestStatus::Pending.is_approved());
        assert!(!OrganizationJoinRequestStatus::Pending.is_rejected());
        assert!(!OrganizationJoinRequestStatus::Pending.is_canceled());
    }

    #[test]
    fn approved_status_is_approved() {
        assert!(!OrganizationJoinRequestStatus::Approved.is_pending());
        assert!(OrganizationJoinRequestStatus::Approved.is_approved());
        assert!(!OrganizationJoinRequestStatus::Approved.is_rejected());
        assert!(!OrganizationJoinRequestStatus::Approved.is_canceled());
    }

    #[test]
    fn rejected_status_is_rejected() {
        assert!(!OrganizationJoinRequestStatus::Rejected.is_pending());
        assert!(!OrganizationJoinRequestStatus::Rejected.is_approved());
        assert!(OrganizationJoinRequestStatus::Rejected.is_rejected());
        assert!(!OrganizationJoinRequestStatus::Rejected.is_canceled());
    }

    #[test]
    fn canceled_status_is_canceled() {
        assert!(!OrganizationJoinRequestStatus::Canceled.is_pending());
        assert!(!OrganizationJoinRequestStatus::Canceled.is_approved());
        assert!(!OrganizationJoinRequestStatus::Canceled.is_rejected());
        assert!(OrganizationJoinRequestStatus::Canceled.is_canceled());
    }
}
