use banking_ledger_domain::onchain::EvmAddress;

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
