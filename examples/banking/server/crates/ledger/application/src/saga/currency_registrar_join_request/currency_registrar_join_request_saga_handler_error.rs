use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the currency registrar join request saga handlers.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestSagaHandlerError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Context(#[from] SagaContextError),
}
