use thiserror::Error;

use appletheia_application::command::{
    CommandHashError, CommandNameOwnedError, CommandOutboxAttemptCountError, CommandOutboxIdError,
    CommandOutboxRelayInstanceError,
};

#[derive(Debug, Error)]
pub enum PgCommandOutboxRowError {
    #[error("command outbox id error: {0}")]
    OutboxId(#[from] CommandOutboxIdError),

    #[error("command name error: {0}")]
    CommandName(#[from] CommandNameOwnedError),

    #[error("command hash error: {0}")]
    CommandHash(#[from] CommandHashError),

    #[error("attempt count error: {0}")]
    AttemptCount(#[from] CommandOutboxAttemptCountError),

    #[error("lease owner error: {0}")]
    LeaseOwner(#[from] CommandOutboxRelayInstanceError),

    #[error("context deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("command outbox row contained inconsistent lease state")]
    InconsistentLeaseState,
}
