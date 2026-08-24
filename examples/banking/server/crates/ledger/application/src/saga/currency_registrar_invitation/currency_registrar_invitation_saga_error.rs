use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

/// Represents errors returned by the currency registrar invitation saga.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarInvitationSagaError {
    #[error("failed to decode event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append saga command")]
    AppendCommand(#[from] SagaInstanceError),
}
