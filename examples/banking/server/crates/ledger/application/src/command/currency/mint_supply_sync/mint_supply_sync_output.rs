use serde::{Deserialize, Serialize};

/// Returned after a currency mint supply sync request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintSupplySyncOutput;
