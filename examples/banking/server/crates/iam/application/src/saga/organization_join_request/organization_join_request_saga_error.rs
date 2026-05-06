use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaInstanceError;
use thiserror::Error;

/// Represents errors returned by the organization join request saga.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestSagaError {
    #[error("failed to decode event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append saga command")]
    AppendCommand(#[from] SagaInstanceError),
}
