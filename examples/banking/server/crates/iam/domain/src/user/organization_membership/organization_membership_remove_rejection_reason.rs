use serde::{Deserialize, Serialize};

/// Describes why removing a user organization membership was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipRemoveRejectionReason {
    Removed,
    Inactive,
    OrganizationRemoved,
    NotMember,
}
