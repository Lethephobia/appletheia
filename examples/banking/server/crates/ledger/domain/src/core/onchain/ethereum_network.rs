use serde::{Deserialize, Serialize};

use super::EvmChainId;

/// Identifies a supported Ethereum network.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum EthereumNetwork {
    Mainnet,
    Sepolia,
    Hoodi,
    Local { chain_id: EvmChainId },
}

impl EthereumNetwork {
    /// Returns the EVM chain ID.
    pub const fn chain_id(self) -> EvmChainId {
        match self {
            Self::Mainnet => EvmChainId::new(1),
            Self::Sepolia => EvmChainId::new(11_155_111),
            Self::Hoodi => EvmChainId::new(560_048),
            Self::Local { chain_id } => chain_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EthereumNetwork, EvmChainId};

    #[test]
    fn public_networks_have_their_canonical_chain_ids() {
        assert_eq!(EthereumNetwork::Mainnet.chain_id(), EvmChainId::new(1));
        assert_eq!(
            EthereumNetwork::Sepolia.chain_id(),
            EvmChainId::new(11_155_111)
        );
        assert_eq!(EthereumNetwork::Hoodi.chain_id(), EvmChainId::new(560_048));
    }
}
