use std::error::Error;

use thiserror::Error;

/// Errors returned while encrypting or decrypting exchange grants.
#[derive(Debug, Error)]
pub enum AuthTokenExchangeGrantCipherError {
    #[error("exchange grant cipher backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync + 'static>),
}
