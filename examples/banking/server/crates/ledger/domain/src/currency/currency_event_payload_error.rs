use thiserror::Error;

/// Describes why a Currency event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum CurrencyEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
