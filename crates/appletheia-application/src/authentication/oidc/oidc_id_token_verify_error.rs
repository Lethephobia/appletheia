use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OidcIdTokenVerifyError {
    #[error("invalid id token")]
    InvalidIdToken,

    #[error("backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync>),
}
