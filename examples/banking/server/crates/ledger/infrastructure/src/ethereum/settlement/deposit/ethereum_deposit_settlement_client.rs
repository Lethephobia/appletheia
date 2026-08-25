use banking_ledger_application::{
    EthereumDepositSettlementPrepareRequest, EthereumDepositSettlementVerifyRequest, EvmCallData,
};
use banking_ledger_domain::core::{EthereumNetwork, EvmAddress};

#[allow(async_fn_in_trait)]
pub trait EthereumDepositSettlementClient: Send + Sync {
    async fn prepare_deposit(
        &self,
        network: EthereumNetwork,
        settlement_contract: &EvmAddress,
        request: EthereumDepositSettlementPrepareRequest,
    ) -> Result<EvmCallData, Box<dyn std::error::Error + Send + Sync>>;

    async fn verify_deposit(
        &self,
        network: EthereumNetwork,
        settlement_contract: &EvmAddress,
        request: EthereumDepositSettlementVerifyRequest,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
