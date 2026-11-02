use serde::{Deserialize, Serialize};

use super::{EvmUserOperationRequest, SolanaPreparedDepositTransaction};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DepositSettlementPreparation {
    Solana(SolanaPreparedDepositTransaction),
    // Boxed to keep the enum small because this variant is substantially larger than Solana.
    Ethereum(Box<EvmUserOperationRequest>),
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::core::{EvmAddress, EvmChainId, EvmTokenOwnerAddress};
    use serde_json::json;

    use crate::settlement::{
        DepositSettlementPreparation, EvmCallData, EvmQuantity, EvmUserOperation,
        EvmUserOperationRequest, SolanaPreparedDepositTransaction,
    };

    #[test]
    fn serializes_each_chain_specific_preparation() {
        let solana = DepositSettlementPreparation::Solana(
            SolanaPreparedDepositTransaction::from_bytes(b"solana".to_vec()),
        );
        let ethereum =
            DepositSettlementPreparation::Ethereum(Box::new(EvmUserOperationRequest::new(
                EvmChainId::new(11_155_111),
                EvmAddress::from_bytes([3; 20]),
                EvmUserOperation {
                    sender: EvmTokenOwnerAddress::new(EvmAddress::from_bytes([1; 20])),
                    nonce: EvmQuantity::from_u64(1),
                    call_data: EvmCallData::from_bytes(vec![0x12, 0xab]),
                    call_gas_limit: EvmQuantity::from_u64(2),
                    verification_gas_limit: EvmQuantity::from_u64(3),
                    pre_verification_gas: EvmQuantity::from_u64(4),
                    max_fee_per_gas: EvmQuantity::from_u64(5),
                    max_priority_fee_per_gas: EvmQuantity::from_u64(6),
                    paymaster: EvmAddress::from_bytes([2; 20]),
                    paymaster_verification_gas_limit: EvmQuantity::from_u64(7),
                    paymaster_post_op_gas_limit: EvmQuantity::from_u64(8),
                    paymaster_data: EvmCallData::from_bytes(vec![0xcd]),
                },
            )));

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
                    "entry_point": "0x0303030303030303030303030303030303030303",
                    "user_operation": {
                        "sender": "0x0101010101010101010101010101010101010101",
                        "nonce": "0x1",
                        "callData": "0x12ab",
                        "callGasLimit": "0x2",
                        "verificationGasLimit": "0x3",
                        "preVerificationGas": "0x4",
                        "maxFeePerGas": "0x5",
                        "maxPriorityFeePerGas": "0x6",
                        "paymaster": "0x0202020202020202020202020202020202020202",
                        "paymasterVerificationGasLimit": "0x7",
                        "paymasterPostOpGasLimit": "0x8",
                        "paymasterData": "0xcd"
                    }
                }
            })
        );
    }
}
