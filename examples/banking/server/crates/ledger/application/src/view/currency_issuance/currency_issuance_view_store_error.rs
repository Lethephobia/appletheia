use thiserror::Error;

/// Represents errors returned by currency issuance view stores.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceViewStoreError {
    #[error("currency issuance view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
