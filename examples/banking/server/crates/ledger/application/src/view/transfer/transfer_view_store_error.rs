use thiserror::Error;

/// Represents errors returned by transfer view stores.
#[derive(Debug, Error)]
pub enum TransferViewStoreError {
    #[error("transfer view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
