use serde::{Deserialize, Serialize};

/// Returned after completing a transfer.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferCompleteOutput;
