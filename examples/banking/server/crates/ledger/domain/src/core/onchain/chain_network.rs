use serde::{Deserialize, Serialize};

use super::{EthereumNetwork, SolanaNetwork};

/// Identifies the concrete blockchain network used for settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "chain", content = "network", rename_all = "snake_case")]
pub enum ChainNetwork {
    Solana(SolanaNetwork),
    Ethereum(EthereumNetwork),
}

impl ChainNetwork {
    /// Returns the stable chain name used in unique values and configuration.
    pub const fn chain_name(self) -> &'static str {
        match self {
            Self::Solana(_) => "solana",
            Self::Ethereum(_) => "ethereum",
        }
    }

    /// Returns a stable network identifier.
    pub fn network_name(self) -> String {
        match self {
            Self::Solana(SolanaNetwork::Mainnet) => "mainnet".to_owned(),
            Self::Solana(SolanaNetwork::Devnet) => "devnet".to_owned(),
            Self::Solana(SolanaNetwork::Testnet) => "testnet".to_owned(),
            Self::Solana(SolanaNetwork::Localnet) => "localnet".to_owned(),
            Self::Ethereum(network) => network.chain_id().to_string(),
        }
    }
}
