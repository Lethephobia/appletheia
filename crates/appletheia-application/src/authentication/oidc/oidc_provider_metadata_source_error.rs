use std::error::Error;

use thiserror::Error;

use super::OidcIssuerUrl;

#[derive(Debug, Error)]
pub enum OidcProviderMetadataSourceError {
    #[error("provider metadata issuer mismatch: expected={expected} actual={actual}")]
    IssuerMismatch {
        expected: Box<OidcIssuerUrl>,
        actual: Box<OidcIssuerUrl>,
    },

    #[error("backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync>),
}
