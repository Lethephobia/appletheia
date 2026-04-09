use appletheia::application::command::CommandOwnedError;
use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

/// Represents errors returned by the currency issuance saga.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceSagaError {
    #[error("failed to decode event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append currency issuance saga command")]
    SagaInstance(#[from] SagaInstanceError),

    #[error("failed to build currency issuance saga owned command")]
    CommandOwned(#[from] CommandOwnedError),
}
