use thiserror::Error;

/// Represents errors returned by transfer projection stores.
#[derive(Debug, Error)]
pub enum TransferProjectionStoreError {
    #[error("transfer projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
