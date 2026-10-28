use appletheia::application::command::CommandFailureEnvelopeError;
use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaContextError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationOldPictureObjectDeletionSagaHandlerError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    CommandFailureEnvelope(#[from] CommandFailureEnvelopeError),

    #[error(transparent)]
    Context(#[from] SagaContextError),

    #[error("unexpected organization old picture object deletion saga event")]
    UnexpectedEvent,
}
