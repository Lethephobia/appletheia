use appletheia::application::Retryability;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MintSupplySynchronizerError {
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for MintSupplySynchronizerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Backend(_) => true,
        }
    }
}
