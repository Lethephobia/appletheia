use std::error::Error;

use thiserror::Error;

/// Describes why user info could not be fetched.
#[derive(Debug, Error)]
pub enum OidcUserInfoClientError {
    #[error("backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync>),
}
