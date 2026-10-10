use appletheia::application::command::CommandFailureEnvelopeError;
use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaContextError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamMembershipCleanupSagaHandlerError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Context(#[from] SagaContextError),

    #[error(transparent)]
    CommandFailureEnvelope(#[from] CommandFailureEnvelopeError),
}
