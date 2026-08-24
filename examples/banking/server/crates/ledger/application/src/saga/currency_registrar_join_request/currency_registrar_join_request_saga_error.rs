use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

/// Represents errors returned by the currency registrar join request saga.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestSagaError {
    #[error("failed to decode event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append saga command")]
    AppendCommand(#[from] SagaInstanceError),
}
