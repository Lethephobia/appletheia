use appletheia_application::command::{
    CommandAttemptCountError, CommandFailureIdError, CommandNameOwnedError,
};
use appletheia_application::outbox::command_failure::CommandFailureOutboxIdError;
use appletheia_application::outbox::{OutboxAttemptCountError, OutboxRelayInstanceError};
use appletheia_application::saga::{
    SagaInstanceIdError, SagaNameOwnedError, SerializedSagaStepError,
};
use thiserror::Error;

/// Reports invalid state loaded from the command-failure outbox.
#[derive(Debug, Error)]
pub enum PgCommandFailureOutboxRowError {
    #[error(transparent)]
    Id(#[from] CommandFailureOutboxIdError),
    #[error(transparent)]
    FailureId(#[from] CommandFailureIdError),
    #[error(transparent)]
    CommandName(#[from] CommandNameOwnedError),
    #[error(transparent)]
    SagaName(#[from] SagaNameOwnedError),
    #[error(transparent)]
    SagaInstanceId(#[from] SagaInstanceIdError),
    #[error(transparent)]
    SagaStep(#[from] SerializedSagaStepError),
    #[error(transparent)]
    CommandAttemptCount(#[from] CommandAttemptCountError),
    #[error(transparent)]
    OutboxAttemptCount(#[from] OutboxAttemptCountError),
    #[error(transparent)]
    LeaseOwner(#[from] OutboxRelayInstanceError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("unknown command terminal reason: {0}")]
    TerminalReason(String),
    #[error("outbox row contained inconsistent lease state")]
    InconsistentLeaseState,
}
