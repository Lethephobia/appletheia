use thiserror::Error;

/// Describes why a payout destination event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum PayoutDestinationEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
