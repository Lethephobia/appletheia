use thiserror::Error;

/// Describes why a transfer event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum TransferEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
