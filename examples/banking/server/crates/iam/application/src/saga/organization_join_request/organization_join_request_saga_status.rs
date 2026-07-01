use serde::{Deserialize, Serialize};

/// Describes progress for the organization join request saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationJoinRequestSagaStatus {
    MembershipGrantRequested,
    MembershipGranted,
    AlreadyMember,
    Failed,
}
