use super::{
    DepositSettlementPreparerError, SolanaDepositSettlementPreparation,
    SolanaDepositSettlementPreparationRequest,
};

#[allow(async_fn_in_trait)]
pub trait SolanaDepositSettlementPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: SolanaDepositSettlementPreparationRequest,
    ) -> Result<SolanaDepositSettlementPreparation, DepositSettlementPreparerError>;
}
