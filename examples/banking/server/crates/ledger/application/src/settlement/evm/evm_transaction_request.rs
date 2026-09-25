use banking_ledger_domain::core::{EvmAddress, EvmChainId};

use super::EvmCallData;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvmTransactionRequest {
    chain_id: EvmChainId,
    sender: EvmAddress,
    target: EvmAddress,
    call_data: EvmCallData,
}

impl EvmTransactionRequest {
    pub fn new(
        chain_id: EvmChainId,
        sender: EvmAddress,
        target: EvmAddress,
        call_data: EvmCallData,
    ) -> Self {
        Self {
            chain_id,
            sender,
            target,
            call_data,
        }
    }

    pub const fn chain_id(&self) -> EvmChainId {
        self.chain_id
    }

    pub const fn sender(&self) -> EvmAddress {
        self.sender
    }

    pub const fn target(&self) -> EvmAddress {
        self.target
    }

    pub const fn call_data(&self) -> &EvmCallData {
        &self.call_data
    }
}
