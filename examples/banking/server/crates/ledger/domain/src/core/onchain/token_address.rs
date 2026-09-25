use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::{ChainNetwork, EvmTokenContractAddress, SolanaMintAccountAddress};

/// Identifies the external token used by a settlement binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "chain", content = "address", rename_all = "snake_case")]
pub enum TokenAddress {
    Solana(SolanaMintAccountAddress),
    Ethereum(EvmTokenContractAddress),
}

impl TokenAddress {
    /// Returns whether this address belongs to the same chain as the network.
    pub const fn matches_network(&self, network: ChainNetwork) -> bool {
        matches!(
            (self, network),
            (Self::Solana(_), ChainNetwork::Solana) | (Self::Ethereum(_), ChainNetwork::Ethereum)
        )
    }
}

impl Display for TokenAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Solana(address) => Display::fmt(address, formatter),
            Self::Ethereum(address) => Display::fmt(address, formatter),
        }
    }
}
