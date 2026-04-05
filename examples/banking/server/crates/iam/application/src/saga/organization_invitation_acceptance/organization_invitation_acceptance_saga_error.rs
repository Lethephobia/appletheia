use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaAppendCommandError;
use thiserror::Error;

/// Represents errors returned by the organization invitation acceptance saga.
#[derive(Debug, Error)]
pub enum OrganizationInvitationAcceptanceSagaError {
    #[error("failed to decode event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append saga command")]
    AppendCommand(#[from] SagaAppendCommandError),
}
