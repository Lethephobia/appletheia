use thiserror::Error;

/// Describes why an currency registrar invitation event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarInvitationEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
