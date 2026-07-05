use serde::{Deserialize, Serialize};

/// Returned after the on-chain ledger backend configuration request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnchainConfigureOutput;
