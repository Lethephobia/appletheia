use super::{
    EthereumDepositSettlementTransactionPreparation,
    EthereumDepositSettlementTransactionPreparationRequest,
    EthereumDepositSettlementTransactionPreparerError,
};

#[allow(async_fn_in_trait)]
pub trait EthereumDepositSettlementTransactionPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: EthereumDepositSettlementTransactionPreparationRequest,
    ) -> Result<
        EthereumDepositSettlementTransactionPreparation,
        EthereumDepositSettlementTransactionPreparerError,
    >;
}
