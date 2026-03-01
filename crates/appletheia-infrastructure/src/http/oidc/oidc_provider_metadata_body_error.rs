use thiserror::Error;

#[derive(Debug, Error)]
pub enum OidcProviderMetadataBodyError {
    #[error("invalid provider metadata json")]
    InvalidJson(#[source] serde_json::Error),
}
