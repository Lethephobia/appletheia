use thiserror::Error;

/// Describes why a deposit event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum DepositEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
