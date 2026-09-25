use serde::{Deserialize, Serialize};

/// Describes why creating an organization membership was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipCreateRejectionReason {
    OrganizationRemoved,
    UserRemoved,
    UserInactive,
    AlreadyMember,
}
