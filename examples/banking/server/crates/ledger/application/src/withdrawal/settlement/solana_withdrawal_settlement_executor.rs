use super::{
    SolanaWithdrawalSettlementExecution, SolanaWithdrawalSettlementRequest,
    WithdrawalSettlementExecutorError,
};

#[allow(async_fn_in_trait)]
pub trait SolanaWithdrawalSettlementExecutor: Send + Sync {
    async fn execute(
        &self,
        request: SolanaWithdrawalSettlementRequest,
    ) -> Result<SolanaWithdrawalSettlementExecution, WithdrawalSettlementExecutorError>;
}
