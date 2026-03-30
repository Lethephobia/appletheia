use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting currency-definition owner relationships.
#[derive(Debug, Error)]
pub enum CurrencyDefinitionOwnerRelationshipProjectorError {
    #[error("failed to decode currency definition event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to persist currency definition owner relationship")]
    RelationshipStore(#[from] RelationshipStoreError),
}
