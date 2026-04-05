use serde::{Deserialize, Serialize};

/// The output returned after deactivating an organization membership.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipDeactivateOutput;
