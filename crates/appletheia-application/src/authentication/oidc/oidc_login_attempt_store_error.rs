use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OidcLoginAttemptStoreError {
    #[error("login attempt not found")]
    NotFound,

    #[error("login attempt already consumed")]
    AlreadyConsumed,

    #[error("login attempt expired")]
    Expired,

    #[error("backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync>),
}
