use serde::{Deserialize, Serialize};

/// Identifies a supported Solana network.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SolanaNetwork {
    Mainnet,
    Devnet,
    Testnet,
    Localnet,
}

#[cfg(test)]
mod tests {
    use super::SolanaNetwork;

    #[test]
    fn testnet_has_a_stable_serialized_name() {
        assert_eq!(
            serde_json::to_string(&SolanaNetwork::Testnet).expect("network should serialize"),
            "\"testnet\""
        );
    }
}
