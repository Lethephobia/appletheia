use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyOldImageObjectDeletionSagaError {
    #[error("failed to decode currency event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append currency old image object deletion saga command")]
    SagaInstance(#[from] SagaInstanceError),

    #[error("unexpected currency old image object deletion saga event")]
    UnexpectedEvent,
}
