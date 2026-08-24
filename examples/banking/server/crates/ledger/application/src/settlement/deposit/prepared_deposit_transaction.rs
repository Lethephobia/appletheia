use serde::{Deserialize, Serialize};

use super::{EvmPreparedDepositTransaction, SolanaPreparedDepositTransaction};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum PreparedDepositTransaction {
    Solana(SolanaPreparedDepositTransaction),
    Ethereum(EvmPreparedDepositTransaction),
}

impl PreparedDepositTransaction {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Solana(transaction) => transaction.as_bytes(),
            Self::Ethereum(transaction) => transaction.as_bytes(),
        }
    }

    pub fn to_base64(&self) -> String {
        match self {
            Self::Solana(transaction) => transaction.to_base64(),
            Self::Ethereum(transaction) => transaction.to_base64(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        EvmPreparedDepositTransaction, PreparedDepositTransaction, SolanaPreparedDepositTransaction,
    };

    #[test]
    fn serializes_the_chain_and_base64_encoded_bytes() {
        let solana = PreparedDepositTransaction::Solana(
            SolanaPreparedDepositTransaction::from_bytes(b"solana".to_vec()),
        );
        let ethereum = PreparedDepositTransaction::Ethereum(
            EvmPreparedDepositTransaction::from_bytes(b"ethereum".to_vec()),
        );

        assert_eq!(
            serde_json::to_value(solana).expect("Solana transaction should serialize"),
            json!({ "type": "solana", "data": "c29sYW5h" })
        );
        assert_eq!(
            serde_json::to_value(ethereum).expect("Ethereum transaction should serialize"),
            json!({ "type": "ethereum", "data": "ZXRoZXJldW0=" })
        );
    }

    #[test]
    fn deserializes_base64_encoded_bytes_for_each_chain() {
        let solana: PreparedDepositTransaction = serde_json::from_value(json!({
            "type": "solana",
            "data": "c29sYW5h"
        }))
        .expect("Solana transaction should deserialize");
        let ethereum: PreparedDepositTransaction = serde_json::from_value(json!({
            "type": "ethereum",
            "data": "ZXRoZXJldW0="
        }))
        .expect("Ethereum transaction should deserialize");

        assert_eq!(solana.as_bytes(), b"solana");
        assert_eq!(ethereum.as_bytes(), b"ethereum");
    }
}
