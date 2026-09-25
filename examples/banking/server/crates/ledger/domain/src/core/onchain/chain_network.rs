use serde::{Deserialize, Serialize};

/// Identifies the blockchain used for settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChainNetwork {
    Solana,
    Ethereum,
}

impl ChainNetwork {
    /// Returns the stable chain name used in unique values and configuration.
    pub const fn chain_name(self) -> &'static str {
        match self {
            Self::Solana => "solana",
            Self::Ethereum => "ethereum",
        }
    }
}
