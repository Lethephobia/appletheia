use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting role assignee relationships.
#[derive(Debug, Error)]
pub enum RoleAssigneeRelationshipProjectorError {
    #[error("event envelope is invalid")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("relationship store failed")]
    RelationshipStore(#[from] RelationshipStoreError),
}
