use serde::{Deserialize, Serialize};

/// Returned after an organization removal request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationRemoveOutput;
