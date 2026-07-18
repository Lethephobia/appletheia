use std::error::Error;

use thiserror::Error;

use crate::Retryability;

/// Errors returned by OIDC continuation persistence backends.
#[derive(Debug, Error)]
pub enum OidcContinuationStoreError {
    #[error("oidc continuation not found")]
    NotFound,

    #[error("oidc continuation already consumed")]
    AlreadyConsumed,

    #[error("oidc continuation expired")]
    Expired,

    #[error("oidc continuation store backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync + 'static>),
}

impl Retryability for OidcContinuationStoreError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::NotFound | Self::AlreadyConsumed | Self::Expired => false,
            Self::Backend(_) => true,
        }
    }
}
