use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned by the organization join request organization relationship projector.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestOrganizationRelationshipProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("relationship store failed")]
    RelationshipStore(#[from] RelationshipStoreError),
}
