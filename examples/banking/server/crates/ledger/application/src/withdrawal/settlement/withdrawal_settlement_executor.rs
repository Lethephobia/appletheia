use super::{
    WithdrawalSettlementExecution, WithdrawalSettlementExecutorError, WithdrawalSettlementRequest,
};

#[allow(async_fn_in_trait)]
pub trait WithdrawalSettlementExecutor: Send + Sync {
    async fn execute(
        &self,
        request: WithdrawalSettlementRequest,
    ) -> Result<WithdrawalSettlementExecution, WithdrawalSettlementExecutorError>;
}
