use thiserror::Error;

/// Describes why a wallet bookmark event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum WalletBookmarkEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
