use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use banking_iam_domain::RoleNameError;
use thiserror::Error;

/// Represents errors returned while projecting initial status manager relationships.
#[derive(Debug, Error)]
pub enum UserStatusManagerRelationshipProjectorError {
    #[error("failed to decode user event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("admin role name is invalid")]
    RoleName(#[from] RoleNameError),

    #[error("failed to persist user status manager relationship")]
    RelationshipStore(#[from] RelationshipStoreError),
}
