use thiserror::Error;

/// Represents errors returned by currency view stores.
#[derive(Debug, Error)]
pub enum CurrencyViewStoreError {
    #[error("currency view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
