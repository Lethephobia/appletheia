use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OidcIdTokenVerifierError {
    #[error("invalid id token")]
    InvalidIdToken {
        #[source]
        source: Option<Box<dyn Error + Send + Sync>>,
    },

    #[error("backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync>),
}

impl OidcIdTokenVerifierError {
    pub fn invalid_id_token() -> Self {
        Self::InvalidIdToken { source: None }
    }

    pub fn invalid_id_token_with_source(source: impl Error + Send + Sync + 'static) -> Self {
        Self::InvalidIdToken {
            source: Some(Box::new(source)),
        }
    }
}
