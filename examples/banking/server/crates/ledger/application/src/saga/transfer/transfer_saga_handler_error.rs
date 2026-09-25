use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the transfer saga handlers.
#[derive(Debug, Error)]
pub enum TransferSagaHandlerError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Context(#[from] SagaContextError),
}
