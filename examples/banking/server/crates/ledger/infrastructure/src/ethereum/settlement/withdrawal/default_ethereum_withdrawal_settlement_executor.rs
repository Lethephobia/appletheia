use banking_ledger_application::{
    EthereumWithdrawalSettlementExecution, EthereumWithdrawalSettlementExecutor,
    EthereumWithdrawalSettlementRequest, WithdrawalSettlementExecutorError,
};
use banking_ledger_domain::core::EvmAddress;

use super::EthereumWithdrawalSettlementClient;

pub struct DefaultEthereumWithdrawalSettlementExecutor<C>
where
    C: EthereumWithdrawalSettlementClient,
{
    client: C,
    settlement_contract: EvmAddress,
}

impl<C> DefaultEthereumWithdrawalSettlementExecutor<C>
where
    C: EthereumWithdrawalSettlementClient,
{
    pub fn new(client: C, settlement_contract: EvmAddress) -> Self {
        Self {
            client,
            settlement_contract,
        }
    }
}

impl<C> EthereumWithdrawalSettlementExecutor for DefaultEthereumWithdrawalSettlementExecutor<C>
where
    C: EthereumWithdrawalSettlementClient,
{
    async fn execute(
        &self,
        request: EthereumWithdrawalSettlementRequest,
    ) -> Result<EthereumWithdrawalSettlementExecution, WithdrawalSettlementExecutorError> {
        let network = request.network();
        let transaction_id = self
            .client
            .execute_withdrawal(network, &self.settlement_contract, request)
            .await
            .map_err(WithdrawalSettlementExecutorError::Backend)?;
        Ok(EthereumWithdrawalSettlementExecution { transaction_id })
    }
}
