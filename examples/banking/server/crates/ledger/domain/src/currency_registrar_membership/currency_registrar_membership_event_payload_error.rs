use thiserror::Error;

/// Describes why a CurrencyRegistrarMembership event payload cannot be serialized.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarMembershipEventPayloadError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
