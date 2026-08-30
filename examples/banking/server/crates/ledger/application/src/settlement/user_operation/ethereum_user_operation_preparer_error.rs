use appletheia::application::Retryability;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EthereumUserOperationPreparerError {
    #[error("EVM transaction and Ethereum UserOperation providers use different chain IDs")]
    InconsistentChainId,
    #[error("Ethereum UserOperation backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for EthereumUserOperationPreparerError {
    fn is_retryable(&self) -> bool {
        matches!(self, Self::Backend(_))
    }
}
