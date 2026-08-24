use appletheia::application::Retryability;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WithdrawalSettlementExecutorError {
    #[error("withdrawal settlement values belong to different chains")]
    InconsistentChainValues,
    #[error("withdrawal amount cannot be represented exactly by the selected token")]
    InvalidAmount,
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for WithdrawalSettlementExecutorError {
    fn is_retryable(&self) -> bool {
        matches!(self, Self::Backend(_))
    }
}
