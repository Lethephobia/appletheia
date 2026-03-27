use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting initial owner relationships.
#[derive(Debug, Error)]
pub enum UserOwnerRelationshipProjectorError {
    #[error("failed to decode user event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to persist user owner relationship")]
    RelationshipStore(#[from] RelationshipStoreError),
}
