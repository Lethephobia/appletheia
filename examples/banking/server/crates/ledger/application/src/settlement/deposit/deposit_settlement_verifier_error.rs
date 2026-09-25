use appletheia::application::Retryability;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DepositSettlementVerifierError {
    #[error("deposit settlement values belong to different chains")]
    InconsistentChainValues,
    #[error("deposit amount cannot be represented exactly by the selected token")]
    InvalidAmount,
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for DepositSettlementVerifierError {
    fn is_retryable(&self) -> bool {
        matches!(self, Self::Backend(_))
    }
}
