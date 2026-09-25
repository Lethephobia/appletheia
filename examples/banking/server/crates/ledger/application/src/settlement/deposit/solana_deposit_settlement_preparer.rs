use super::{
    DepositSettlementPreparerError, SolanaDepositSettlementPreparation,
    SolanaDepositSettlementPrepareRequest,
};

#[allow(async_fn_in_trait)]
pub trait SolanaDepositSettlementPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: SolanaDepositSettlementPrepareRequest,
    ) -> Result<SolanaDepositSettlementPreparation, DepositSettlementPreparerError>;
}
