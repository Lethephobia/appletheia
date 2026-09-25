use super::{
    EthereumUserOperationPreparation, EthereumUserOperationPrepareRequest,
    EthereumUserOperationPreparerError,
};

#[allow(async_fn_in_trait)]
pub trait EthereumUserOperationPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: EthereumUserOperationPrepareRequest,
    ) -> Result<EthereumUserOperationPreparation, EthereumUserOperationPreparerError>;
}
