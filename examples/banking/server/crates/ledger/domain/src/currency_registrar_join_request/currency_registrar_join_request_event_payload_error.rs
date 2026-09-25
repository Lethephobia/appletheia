use thiserror::Error;

/// Describes why an currency registrar join request event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
