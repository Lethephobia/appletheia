use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::{ChainNetwork, EvmTransactionHash, SolanaTransactionSignature};

/// Identifies a verified transaction on a supported blockchain.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "chain", content = "transaction_id", rename_all = "snake_case")]
pub enum OnchainTransactionId {
    Solana(SolanaTransactionSignature),
    Ethereum(EvmTransactionHash),
}

impl OnchainTransactionId {
    /// Returns whether this transaction belongs to the same chain as the network.
    pub const fn matches_network(&self, network: ChainNetwork) -> bool {
        matches!(
            (self, network),
            (Self::Solana(_), ChainNetwork::Solana(_))
                | (Self::Ethereum(_), ChainNetwork::Ethereum(_))
        )
    }

    /// Returns the decoded transaction-identifier bytes.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Solana(signature) => signature.as_bytes(),
            Self::Ethereum(hash) => hash.as_bytes(),
        }
    }
}

impl Display for OnchainTransactionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Solana(signature) => Display::fmt(signature, formatter),
            Self::Ethereum(hash) => Display::fmt(hash, formatter),
        }
    }
}
