use thiserror::Error;

use crate::Retryability;

#[derive(Debug, Error)]
pub enum AuthTokenIssuerError {
    #[error("token issue failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Retryability for AuthTokenIssuerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Backend(_) => true,
        }
    }
}
