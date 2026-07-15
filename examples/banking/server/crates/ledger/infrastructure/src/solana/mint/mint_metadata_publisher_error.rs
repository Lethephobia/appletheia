use thiserror::Error;

#[derive(Debug, Error)]
pub enum MintMetadataPublisherError {
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
