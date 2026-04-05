use serde::{Deserialize, Serialize};

/// Returned after an organization name change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeNameOutput;
