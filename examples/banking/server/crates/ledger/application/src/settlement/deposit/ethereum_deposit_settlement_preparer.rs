use super::{
    DepositSettlementPreparerError, EthereumDepositSettlementPreparation,
    EthereumDepositSettlementPrepareRequest,
};

#[allow(async_fn_in_trait)]
pub trait EthereumDepositSettlementPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: EthereumDepositSettlementPrepareRequest,
    ) -> Result<EthereumDepositSettlementPreparation, DepositSettlementPreparerError>;
}
