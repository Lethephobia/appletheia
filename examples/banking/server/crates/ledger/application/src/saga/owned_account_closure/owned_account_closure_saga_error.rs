use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

/// Represents errors returned by the owned account closure saga.
#[derive(Debug, Error)]
pub enum OwnedAccountClosureSagaError {
    #[error("failed to decode event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append owned account closure saga command")]
    SagaInstance(#[from] SagaInstanceError),

    #[error("owned account closure saga state is missing closure id")]
    MissingOwnedAccountClosureId,
}
