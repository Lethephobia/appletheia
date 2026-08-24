use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::{ChainNetwork, EvmTokenOwnerAddress, SolanaTokenOwnerAddress};

/// Identifies an owner of external tokens on a concrete chain.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "chain", content = "address", rename_all = "snake_case")]
pub enum TokenOwnerAddress {
    Solana(SolanaTokenOwnerAddress),
    Ethereum(EvmTokenOwnerAddress),
}

impl TokenOwnerAddress {
    /// Returns whether this address belongs to the same chain as the network.
    pub const fn matches_network(&self, network: ChainNetwork) -> bool {
        matches!(
            (self, network),
            (Self::Solana(_), ChainNetwork::Solana(_))
                | (Self::Ethereum(_), ChainNetwork::Ethereum(_))
        )
    }
}

impl Display for TokenOwnerAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Solana(address) => Display::fmt(address, formatter),
            Self::Ethereum(address) => Display::fmt(address, formatter),
        }
    }
}
