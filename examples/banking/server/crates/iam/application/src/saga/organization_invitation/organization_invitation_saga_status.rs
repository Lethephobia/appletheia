use serde::{Deserialize, Serialize};

/// Describes progress for the organization invitation saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationInvitationSagaStatus {
    MembershipGrantRequested,
    MembershipGranted,
    AlreadyMember,
    Failed,
}
