use serde::{Deserialize, Serialize};

/// The output returned after revoking a role from an organization membership.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipRoleRevokeOutput;
