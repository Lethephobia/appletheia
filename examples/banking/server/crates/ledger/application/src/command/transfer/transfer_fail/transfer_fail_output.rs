use serde::{Deserialize, Serialize};

/// Returned after failing a transfer.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferFailOutput;
