use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of an `OrganizationInvitation`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationInvitationStatus {
    Pending,
    Accepted,
    Declined,
    Canceled,
}

impl OrganizationInvitationStatus {
    /// Returns whether the invitation is pending.
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns whether the invitation is accepted.
    pub fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }

    /// Returns whether the invitation is declined.
    pub fn is_declined(&self) -> bool {
        matches!(self, Self::Declined)
    }

    /// Returns whether the invitation is canceled.
    pub fn is_canceled(&self) -> bool {
        matches!(self, Self::Canceled)
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationInvitationStatus;

    #[test]
    fn pending_status_is_pending() {
        assert!(OrganizationInvitationStatus::Pending.is_pending());
        assert!(!OrganizationInvitationStatus::Pending.is_accepted());
        assert!(!OrganizationInvitationStatus::Pending.is_declined());
        assert!(!OrganizationInvitationStatus::Pending.is_canceled());
    }

    #[test]
    fn accepted_status_is_accepted() {
        assert!(!OrganizationInvitationStatus::Accepted.is_pending());
        assert!(OrganizationInvitationStatus::Accepted.is_accepted());
        assert!(!OrganizationInvitationStatus::Accepted.is_declined());
        assert!(!OrganizationInvitationStatus::Accepted.is_canceled());
    }

    #[test]
    fn declined_status_is_declined() {
        assert!(!OrganizationInvitationStatus::Declined.is_pending());
        assert!(!OrganizationInvitationStatus::Declined.is_accepted());
        assert!(OrganizationInvitationStatus::Declined.is_declined());
        assert!(!OrganizationInvitationStatus::Declined.is_canceled());
    }

    #[test]
    fn canceled_status_is_canceled() {
        assert!(!OrganizationInvitationStatus::Canceled.is_pending());
        assert!(!OrganizationInvitationStatus::Canceled.is_accepted());
        assert!(!OrganizationInvitationStatus::Canceled.is_declined());
        assert!(OrganizationInvitationStatus::Canceled.is_canceled());
    }
}
