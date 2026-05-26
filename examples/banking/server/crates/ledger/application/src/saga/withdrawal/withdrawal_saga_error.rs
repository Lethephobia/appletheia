use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

/// Represents errors returned by the withdrawal saga.
#[derive(Debug, Error)]
pub enum WithdrawalSagaError {
    #[error("failed to decode account event envelope")]
    AccountEventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append withdrawal saga command")]
    SagaInstance(#[from] SagaInstanceError),
}
