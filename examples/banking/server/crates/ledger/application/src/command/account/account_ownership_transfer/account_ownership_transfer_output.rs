use serde::{Deserialize, Serialize};

/// The output returned after transferring account ownership.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountOwnershipTransferOutput;
