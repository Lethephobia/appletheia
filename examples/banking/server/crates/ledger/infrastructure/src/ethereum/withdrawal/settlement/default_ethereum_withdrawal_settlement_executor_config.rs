use banking_ledger_domain::core::EvmAddress;

pub struct DefaultEthereumWithdrawalSettlementExecutorConfig {
    pub(super) settlement_contract: EvmAddress,
}

impl DefaultEthereumWithdrawalSettlementExecutorConfig {
    pub fn new(settlement_contract: EvmAddress) -> Self {
        Self {
            settlement_contract,
        }
    }
}
