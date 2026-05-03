use thiserror::Error;

/// Represents errors returned by currency issuance projection stores.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceProjectionStoreError {
    #[error("currency issuance projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
