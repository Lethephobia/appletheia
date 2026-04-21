use serde::{Deserialize, Serialize};

/// The output returned after granting a role to an organization membership.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipRoleGrantOutput;
