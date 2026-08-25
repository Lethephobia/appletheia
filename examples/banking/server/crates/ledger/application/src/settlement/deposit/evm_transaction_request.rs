use banking_ledger_domain::core::{EvmAddress, EvmChainId, EvmTokenOwnerAddress};
use serde::{Deserialize, Serialize};

use super::EvmCallData;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvmTransactionRequest {
    chain_id: EvmChainId,
    from: EvmTokenOwnerAddress,
    to: EvmAddress,
    call_data: EvmCallData,
}

impl EvmTransactionRequest {
    pub fn new(
        chain_id: EvmChainId,
        from: EvmTokenOwnerAddress,
        to: EvmAddress,
        call_data: EvmCallData,
    ) -> Self {
        Self {
            chain_id,
            from,
            to,
            call_data,
        }
    }

    pub const fn chain_id(&self) -> EvmChainId {
        self.chain_id
    }

    pub const fn from(&self) -> EvmTokenOwnerAddress {
        self.from
    }

    pub const fn to(&self) -> EvmAddress {
        self.to
    }

    pub const fn call_data(&self) -> &EvmCallData {
        &self.call_data
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::core::{EvmAddress, EvmChainId, EvmTokenOwnerAddress};
    use serde_json::json;

    use super::{EvmCallData, EvmTransactionRequest};

    #[test]
    fn serializes_a_wallet_transaction_request() {
        let request = EvmTransactionRequest::new(
            EvmChainId::new(11_155_111),
            EvmTokenOwnerAddress::new(EvmAddress::from_bytes([1; 20])),
            EvmAddress::from_bytes([2; 20]),
            EvmCallData::from_bytes(vec![0x12, 0xab]),
        );

        assert_eq!(
            serde_json::to_value(request).expect("transaction request should serialize"),
            json!({
                "chain_id": 11_155_111,
                "from": "0x0101010101010101010101010101010101010101",
                "to": "0x0202020202020202020202020202020202020202",
                "call_data": "0x12ab"
            })
        );
    }
}
