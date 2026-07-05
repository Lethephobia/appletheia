use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MintMetadataSyncSagaError {
    #[error("failed to decode currency event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append currency mint metadata sync saga command")]
    SagaInstance(#[from] SagaInstanceError),

    #[error("unexpected currency mint metadata sync saga event")]
    UnexpectedEvent,
}
