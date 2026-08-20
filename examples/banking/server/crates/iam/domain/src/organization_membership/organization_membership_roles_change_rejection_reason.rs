use serde::{Deserialize, Serialize};

/// Describes why changing organization membership roles was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipRolesChangeRejectionReason {
    Removed,
    OrganizationRemoved,
}
