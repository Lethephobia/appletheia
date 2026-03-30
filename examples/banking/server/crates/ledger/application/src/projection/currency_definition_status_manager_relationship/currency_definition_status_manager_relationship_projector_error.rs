use appletheia::application::authorization::RelationshipStoreError;
use appletheia::application::event::EventEnvelopeError;
use thiserror::Error;

/// Represents errors returned while projecting currency-definition status manager relationships.
#[derive(Debug, Error)]
pub enum CurrencyDefinitionStatusManagerRelationshipProjectorError {
    #[error("failed to decode currency definition event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to persist currency definition status manager relationship")]
    RelationshipStore(#[from] RelationshipStoreError),
}
