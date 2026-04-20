use serde::{Deserialize, Serialize};

/// The output returned after transferring currency ownership.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyOwnershipTransferOutput;
