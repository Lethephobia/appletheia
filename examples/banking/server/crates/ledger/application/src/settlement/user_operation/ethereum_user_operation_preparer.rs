use super::{
    EthereumUserOperationPreparation, EthereumUserOperationPreparationRequest,
    EthereumUserOperationPreparerError,
};

#[allow(async_fn_in_trait)]
pub trait EthereumUserOperationPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: EthereumUserOperationPreparationRequest,
    ) -> Result<EthereumUserOperationPreparation, EthereumUserOperationPreparerError>;
}
