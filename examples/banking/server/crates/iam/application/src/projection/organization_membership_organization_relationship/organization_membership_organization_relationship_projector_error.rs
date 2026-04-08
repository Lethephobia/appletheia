use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned by the organization membership organization relationship projector.
#[derive(Debug, Error)]
pub enum OrganizationMembershipOrganizationRelationshipProjectorError {
    #[error("relationship store failed")]
    RelationshipStore(#[from] RelationshipStoreError),

    #[error("domain event conversion failed")]
    EventEnvelope(#[from] EventEnvelopeError),
}
