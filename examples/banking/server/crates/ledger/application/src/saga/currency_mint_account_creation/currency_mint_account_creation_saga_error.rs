use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyMintAccountCreationSagaError {
    #[error("failed to decode currency event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append currency mint account creation saga command")]
    SagaInstance(#[from] SagaInstanceError),

    #[error("unexpected currency mint account creation saga event")]
    UnexpectedEvent,
}
