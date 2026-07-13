use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

/// Represents errors returned by the deposit saga.
#[derive(Debug, Error)]
pub enum DepositSagaError {
    #[error("failed to decode deposit event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append deposit saga command")]
    SagaInstance(#[from] SagaInstanceError),
}
