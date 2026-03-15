use std::error::Error;

use thiserror::Error;

/// Errors returned by exchange code persistence backends.
#[derive(Debug, Error)]
pub enum AuthTokenExchangeCodeStoreError {
    #[error("exchange code not found")]
    NotFound,

    #[error("exchange code already consumed")]
    AlreadyConsumed,

    #[error("exchange code expired")]
    Expired,

    #[error("exchange code store backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync + 'static>),
}
