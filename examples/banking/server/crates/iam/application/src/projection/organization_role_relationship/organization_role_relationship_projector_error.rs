use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting organization role relationships.
#[derive(Debug, Error)]
pub enum OrganizationRoleRelationshipProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to persist organization role relationship")]
    RelationshipStore(#[from] RelationshipStoreError),
}
