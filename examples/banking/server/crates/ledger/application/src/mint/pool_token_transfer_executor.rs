use super::{PoolTokenTransferExecutorError, PoolTokenTransferRequest};

#[allow(async_fn_in_trait)]
pub trait PoolTokenTransferExecutor: Send + Sync {
    async fn execute(
        &self,
        request: PoolTokenTransferRequest,
    ) -> Result<(), PoolTokenTransferExecutorError>;
}
