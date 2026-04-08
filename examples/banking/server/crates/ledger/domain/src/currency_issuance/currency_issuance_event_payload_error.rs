use thiserror::Error;

/// Describes why a currency-issuance event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
