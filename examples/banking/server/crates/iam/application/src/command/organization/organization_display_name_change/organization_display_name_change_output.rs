use serde::{Deserialize, Serialize};

/// Returned after an organization display name change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationDisplayNameChangeOutput;
