use serde::{Deserialize, Serialize};

/// Returned after a currency mint metadata sync request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyMintAccountMetadataSyncOutput;
