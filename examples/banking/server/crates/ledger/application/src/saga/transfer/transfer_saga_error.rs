use appletheia::application::command::CommandOwnedError;
use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

/// Represents errors returned by the transfer saga.
#[derive(Debug, Error)]
pub enum TransferSagaError {
    #[error("failed to decode account event envelope")]
    AccountEventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append transfer saga command")]
    SagaInstance(#[from] SagaInstanceError),

    #[error("failed to build transfer saga owned command")]
    CommandOwned(#[from] CommandOwnedError),
}
