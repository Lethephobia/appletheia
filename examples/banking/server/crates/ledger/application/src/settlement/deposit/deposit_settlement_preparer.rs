use super::{
    DepositSettlementPreparation, DepositSettlementPrepareRequest, DepositSettlementPreparerError,
};

#[allow(async_fn_in_trait)]
pub trait DepositSettlementPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: DepositSettlementPrepareRequest,
    ) -> Result<DepositSettlementPreparation, DepositSettlementPreparerError>;
}
