use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting account status-manager relationships.
#[derive(Debug, Error)]
pub enum AccountStatusManagerRelationshipProjectorError {
    #[error("failed to decode account event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to persist account status-manager relationships")]
    RelationshipStore(#[from] RelationshipStoreError),
}
