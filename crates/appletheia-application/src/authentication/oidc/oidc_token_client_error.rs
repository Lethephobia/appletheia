use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OidcTokenClientError {
    #[error("backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync>),
}
