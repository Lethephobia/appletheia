use serde::{Deserialize, Serialize};

/// Describes why granting a user organization membership was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipGrantRejectionReason {
    Removed,
    Inactive,
    OrganizationRemoved,
    AlreadyMember,
    CountLimitExceeded,
}
