use std::error::Error;

use thiserror::Error;

/// Errors returned while evaluating or updating token revocation state.
#[derive(Debug, Error)]
pub enum AuthTokenRevocationError {
    #[error("token revocation backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync + 'static>),
}
