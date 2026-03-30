use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting account owner relationships.
#[derive(Debug, Error)]
pub enum AccountOwnerRelationshipProjectorError {
    #[error("failed to decode account event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to persist account owner relationship")]
    RelationshipStore(#[from] RelationshipStoreError),
}
