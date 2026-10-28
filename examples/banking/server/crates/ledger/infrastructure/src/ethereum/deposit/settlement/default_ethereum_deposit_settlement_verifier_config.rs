use banking_ledger_domain::core::EvmAddress;

pub struct DefaultEthereumDepositSettlementVerifierConfig {
    pub(super) settlement_contract: EvmAddress,
}

impl DefaultEthereumDepositSettlementVerifierConfig {
    pub fn new(settlement_contract: EvmAddress) -> Self {
        Self {
            settlement_contract,
        }
    }
}
