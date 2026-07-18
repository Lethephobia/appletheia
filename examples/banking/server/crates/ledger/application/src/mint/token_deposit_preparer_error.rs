use appletheia::application::Retryability;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenDepositPreparerError {
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for TokenDepositPreparerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Backend(_) => true,
        }
    }
}
