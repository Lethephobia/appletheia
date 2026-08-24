use banking_ledger_application::{
    DepositSettlementVerifierError, EthereumDepositSettlementVerification,
    EthereumDepositSettlementVerifier, EthereumDepositSettlementVerifyRequest,
};
use banking_ledger_domain::core::EvmAddress;

use super::EthereumDepositSettlementClient;

pub struct DefaultEthereumDepositSettlementVerifier<C>
where
    C: EthereumDepositSettlementClient,
{
    client: C,
    settlement_contract: EvmAddress,
}

impl<C> DefaultEthereumDepositSettlementVerifier<C>
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

impl<C> EthereumDepositSettlementVerifier for DefaultEthereumDepositSettlementVerifier<C>
where
    C: EthereumDepositSettlementClient,
{
    async fn verify(
        &self,
        request: EthereumDepositSettlementVerifyRequest,
    ) -> Result<EthereumDepositSettlementVerification, DepositSettlementVerifierError> {
        let network = request.network();
        let transaction_id = request.transaction_id();
        self.client
            .verify_deposit(network, &self.settlement_contract, request)
            .await
            .map_err(DepositSettlementVerifierError::Backend)?;
        Ok(EthereumDepositSettlementVerification { transaction_id })
    }
}
