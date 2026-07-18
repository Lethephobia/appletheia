use appletheia::application::Retryability;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OnchainConfigurerError {
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for OnchainConfigurerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Backend(_) => true,
        }
    }
}
