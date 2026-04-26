use serde::{Deserialize, Serialize};

/// Returned after an organization handle change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationHandleChangeOutput;
