use super::{PoolTokenTransferExecutorError, PoolTokenTransferReceipt, PoolTokenTransferRequest};

/// Executes the external transfer step of a withdrawal workflow.
#[allow(async_fn_in_trait)]
pub trait PoolTokenTransferExecutor: Send + Sync {
    async fn execute(
        &self,
        request: PoolTokenTransferRequest,
    ) -> Result<PoolTokenTransferReceipt, PoolTokenTransferExecutorError>;
}
