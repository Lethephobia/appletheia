use super::{
    EthereumWithdrawalSettlementExecution, EthereumWithdrawalSettlementRequest,
    WithdrawalSettlementExecutorError,
};

#[allow(async_fn_in_trait)]
pub trait EthereumWithdrawalSettlementExecutor: Send + Sync {
    async fn execute(
        &self,
        request: EthereumWithdrawalSettlementRequest,
    ) -> Result<EthereumWithdrawalSettlementExecution, WithdrawalSettlementExecutorError>;
}
