use thiserror::Error;

/// Represents errors returned by account projection stores.
#[derive(Debug, Error)]
pub enum AccountProjectionStoreError {
    #[error("account projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
