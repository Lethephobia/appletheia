use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaAppendCommandError;
use thiserror::Error;

/// Represents errors returned by the transfer saga.
#[derive(Debug, Error)]
pub enum TransferSagaError {
    #[error("failed to decode account event envelope")]
    AccountEventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append transfer saga command")]
    AppendCommand(#[from] SagaAppendCommandError),

    #[error("transfer saga state is incomplete")]
    IncompleteState,
}
