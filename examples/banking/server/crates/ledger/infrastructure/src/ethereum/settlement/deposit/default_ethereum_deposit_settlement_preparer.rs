use banking_ledger_application::{
    DepositSettlementPreparerError, EthereumDepositSettlementPreparation,
    EthereumDepositSettlementPrepareRequest, EthereumDepositSettlementPreparer,
    PreparedDepositTransaction,
};
use banking_ledger_domain::core::EvmAddress;

use super::EthereumDepositSettlementClient;

pub struct DefaultEthereumDepositSettlementPreparer<C>
where
    C: EthereumDepositSettlementClient,
{
    client: C,
    settlement_contract: EvmAddress,
}

impl<C> DefaultEthereumDepositSettlementPreparer<C>
where
    C: EthereumDepositSettlementClient,
{
    pub fn new(client: C, settlement_contract: EvmAddress) -> Self {
        Self {
            client,
            settlement_contract,
        }
    }
}

impl<C> EthereumDepositSettlementPreparer for DefaultEthereumDepositSettlementPreparer<C>
where
    C: EthereumDepositSettlementClient,
{
    async fn prepare(
        &self,
        request: EthereumDepositSettlementPrepareRequest,
    ) -> Result<EthereumDepositSettlementPreparation, DepositSettlementPreparerError> {
        let network = request.network();
        let transaction = self
            .client
            .prepare_deposit(network, &self.settlement_contract, request)
            .await
            .map_err(DepositSettlementPreparerError::Backend)?;
        Ok(EthereumDepositSettlementPreparation {
            transaction: PreparedDepositTransaction::new(transaction),
        })
    }
}
