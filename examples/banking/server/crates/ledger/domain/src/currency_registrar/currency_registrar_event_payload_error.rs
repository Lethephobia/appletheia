use thiserror::Error;

/// Describes why a CurrencyRegistrar event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
