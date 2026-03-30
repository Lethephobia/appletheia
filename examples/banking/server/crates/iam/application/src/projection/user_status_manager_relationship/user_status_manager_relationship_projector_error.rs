use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting initial status manager relationships.
#[derive(Debug, Error)]
pub enum UserStatusManagerRelationshipProjectorError {
    #[error("failed to decode user event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to persist user status manager relationship")]
    RelationshipStore(#[from] RelationshipStoreError),
}
