use banking_ledger_application::EthereumWithdrawalSettlementRequest;
use banking_ledger_domain::core::{EthereumNetwork, EvmAddress, EvmTransactionHash};

#[allow(async_fn_in_trait)]
pub trait EthereumWithdrawalSettlementClient: Send + Sync {
    async fn execute_withdrawal(
        &self,
        network: EthereumNetwork,
        settlement_contract: &EvmAddress,
        request: EthereumWithdrawalSettlementRequest,
    ) -> Result<EvmTransactionHash, Box<dyn std::error::Error + Send + Sync>>;
}
