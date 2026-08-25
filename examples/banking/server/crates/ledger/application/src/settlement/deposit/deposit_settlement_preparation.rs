use serde::{Deserialize, Serialize};

use super::{EvmTransactionRequest, SolanaPreparedDepositTransaction};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DepositSettlementPreparation {
    Solana(SolanaPreparedDepositTransaction),
    Ethereum(EvmTransactionRequest),
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::core::{EvmAddress, EvmChainId, EvmTokenOwnerAddress};
    use serde_json::json;

    use crate::settlement::{
        DepositSettlementPreparation, EvmCallData, EvmTransactionRequest,
        SolanaPreparedDepositTransaction,
    };

    #[test]
    fn serializes_each_chain_specific_preparation() {
        let solana = DepositSettlementPreparation::Solana(
            SolanaPreparedDepositTransaction::from_bytes(b"solana".to_vec()),
        );
        let ethereum = DepositSettlementPreparation::Ethereum(EvmTransactionRequest::new(
            EvmChainId::new(11_155_111),
            EvmTokenOwnerAddress::new(EvmAddress::from_bytes([1; 20])),
            EvmAddress::from_bytes([2; 20]),
            EvmCallData::from_bytes(vec![0x12, 0xab]),
        ));

        assert_eq!(
            serde_json::to_value(solana).expect("Solana preparation should serialize"),
            json!({
                "type": "solana",
                "data": "c29sYW5h"
            })
        );
        assert_eq!(
            serde_json::to_value(ethereum).expect("Ethereum preparation should serialize"),
            json!({
                "type": "ethereum",
                "data": {
                    "chain_id": 11_155_111,
                    "from": "0x0101010101010101010101010101010101010101",
                    "to": "0x0202020202020202020202020202020202020202",
                    "call_data": "0x12ab"
                }
            })
        );
    }
}
