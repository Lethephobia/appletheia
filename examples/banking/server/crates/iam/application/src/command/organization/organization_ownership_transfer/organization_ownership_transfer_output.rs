use serde::{Deserialize, Serialize};

/// The output returned after transferring organization ownership.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationOwnershipTransferOutput;
