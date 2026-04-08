use serde::{Deserialize, Serialize};

/// The output returned after activating an organization membership.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipActivateOutput;
