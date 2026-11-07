use super::{
    DepositSettlementPreparation, DepositSettlementPreparationRequest,
    DepositSettlementPreparerError,
};

#[allow(async_fn_in_trait)]
pub trait DepositSettlementPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: DepositSettlementPreparationRequest,
    ) -> Result<DepositSettlementPreparation, DepositSettlementPreparerError>;
}
