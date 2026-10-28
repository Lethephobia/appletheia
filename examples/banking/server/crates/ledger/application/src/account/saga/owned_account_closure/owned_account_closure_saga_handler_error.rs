use appletheia::application::command::CommandFailureEnvelopeError;
use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaContextError;
use thiserror::Error;

/// Represents errors returned by the owned account closure saga handlers.
#[derive(Debug, Error)]
pub enum OwnedAccountClosureSagaHandlerError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Context(#[from] SagaContextError),

    #[error(transparent)]
    CommandFailureEnvelope(#[from] CommandFailureEnvelopeError),
}
