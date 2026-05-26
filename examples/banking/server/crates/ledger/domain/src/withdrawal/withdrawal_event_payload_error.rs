use thiserror::Error;

/// Describes why a withdrawal event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum WithdrawalEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
