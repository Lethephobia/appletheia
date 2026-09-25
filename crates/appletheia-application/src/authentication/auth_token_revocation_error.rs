use std::error::Error;

use thiserror::Error;

use crate::Retryability;

/// Errors returned while evaluating or updating token revocation state.
#[derive(Debug, Error)]
pub enum AuthTokenRevocationError {
    #[error("token revocation backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync + 'static>),
}

impl Retryability for AuthTokenRevocationError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Backend(_) => true,
        }
    }
}
