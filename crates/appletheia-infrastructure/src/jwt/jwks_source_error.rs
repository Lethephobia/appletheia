use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwksSourceError {
    #[error("backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync>),

    #[error("invalid jwks")]
    InvalidJwks {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
}
