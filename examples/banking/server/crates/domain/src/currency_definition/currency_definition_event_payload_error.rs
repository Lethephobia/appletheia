use thiserror::Error;

/// Describes why a currency-definition event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum CurrencyDefinitionEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
