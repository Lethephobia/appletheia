use appletheia::application::Retryability;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EthereumDepositSettlementTransactionPreparerError {
    #[error("deposit amount cannot be represented exactly by the selected token")]
    InvalidAmount,
    #[error("Ethereum deposit settlement transaction backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for EthereumDepositSettlementTransactionPreparerError {
    fn is_retryable(&self) -> bool {
        matches!(self, Self::Backend(_))
    }
}
