use alloy::signers::local::PrivateKeySigner;
use banking_ledger_domain::core::EvmAddress;
use chrono::Duration;

pub struct DefaultEthereumDepositSettlementTransactionPreparerConfig {
    pub(super) settlement_contract: EvmAddress,
    pub(super) operator: PrivateKeySigner,
    pub(super) operator_signature_ttl: Duration,
}

impl DefaultEthereumDepositSettlementTransactionPreparerConfig {
    pub fn new(
        settlement_contract: EvmAddress,
        operator: PrivateKeySigner,
        operator_signature_ttl: Duration,
    ) -> Self {
        Self {
            settlement_contract,
            operator,
            operator_signature_ttl,
        }
    }
}
