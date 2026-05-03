use thiserror::Error;

/// Represents errors returned by currency projection stores.
#[derive(Debug, Error)]
pub enum CurrencyProjectionStoreError {
    #[error("currency projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
