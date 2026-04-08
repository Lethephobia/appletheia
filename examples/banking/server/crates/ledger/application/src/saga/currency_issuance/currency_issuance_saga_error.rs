use appletheia::application::event::EventEnvelopeError;
use appletheia::application::saga::SagaAppendCommandError;
use thiserror::Error;

/// Represents errors returned by the currency issuance saga.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceSagaError {
    #[error("failed to decode event envelope")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("failed to append currency issuance saga command")]
    AppendCommand(#[from] SagaAppendCommandError),

    #[error("currency issuance saga state is incomplete")]
    IncompleteState,
}
