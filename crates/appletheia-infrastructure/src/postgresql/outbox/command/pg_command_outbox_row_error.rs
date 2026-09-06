use thiserror::Error;

use appletheia_application::command::SerializedCommandError;
use appletheia_application::outbox::{
    OrderingKeyError, OutboxAttemptCountError, OutboxRelayInstanceError,
    command::CommandOutboxIdError,
};
use appletheia_application::saga::{
    SagaInstanceIdError, SagaNameOwnedError, SerializedSagaStepError,
};

#[derive(Debug, Error)]
pub enum PgCommandOutboxRowError {
    #[error("command outbox id error: {0}")]
    OutboxId(#[from] CommandOutboxIdError),

    #[error("command name error: {0}")]
    CommandName(String),

    #[error("payload error: {0}")]
    Payload(#[from] SerializedCommandError),

    #[error("attempt count error: {0}")]
    AttemptCount(#[from] OutboxAttemptCountError),

    #[error("lease owner error: {0}")]
    LeaseOwner(#[from] OutboxRelayInstanceError),

    #[error("ordering key error: {0}")]
    OrderingKey(#[from] OrderingKeyError),

    #[error("json deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("saga name error: {0}")]
    SagaName(#[from] SagaNameOwnedError),

    #[error("saga instance id error: {0}")]
    SagaInstanceId(#[from] SagaInstanceIdError),

    #[error("saga step error: {0}")]
    SagaStep(#[from] SerializedSagaStepError),

    #[error("command outbox row contained an incomplete saga origin")]
    InconsistentSagaOrigin,

    #[error("command outbox row contained inconsistent lease state")]
    InconsistentLeaseState,
}
