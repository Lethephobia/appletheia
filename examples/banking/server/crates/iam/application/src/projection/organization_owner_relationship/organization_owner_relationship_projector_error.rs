use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting initial organization owner relationships.
#[derive(Debug, Error)]
pub enum OrganizationOwnerRelationshipProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to persist organization owner relationship")]
    RelationshipStore(#[from] RelationshipStoreError),
}
