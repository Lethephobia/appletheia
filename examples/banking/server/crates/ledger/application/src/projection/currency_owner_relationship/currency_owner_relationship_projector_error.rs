use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting currency owner relationships.
#[derive(Debug, Error)]
pub enum CurrencyOwnerRelationshipProjectorError {
    #[error("failed to decode currency event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to persist currency owner relationship")]
    RelationshipStore(#[from] RelationshipStoreError),
}
