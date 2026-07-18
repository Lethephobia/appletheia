use appletheia::application::Retryability;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoolTokenTransferExecutorError {
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for PoolTokenTransferExecutorError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Backend(_) => true,
        }
    }
}
