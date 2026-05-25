use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyProvisioningSagaError {
    #[error("failed to decode currency event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append currency provisioning saga command")]
    SagaInstance(#[from] SagaInstanceError),

    #[error("unexpected currency provisioning saga event")]
    UnexpectedEvent,
}
