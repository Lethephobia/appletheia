use super::{
    EthereumDepositSettlementTransactionPreparation,
    EthereumDepositSettlementTransactionPrepareRequest,
    EthereumDepositSettlementTransactionPreparerError,
};

#[allow(async_fn_in_trait)]
pub trait EthereumDepositSettlementTransactionPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: EthereumDepositSettlementTransactionPrepareRequest,
    ) -> Result<
        EthereumDepositSettlementTransactionPreparation,
        EthereumDepositSettlementTransactionPreparerError,
    >;
}
