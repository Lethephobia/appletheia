use thiserror::Error;

use super::PgCommandFailureOutboxRowError;

/// Reports invalid state loaded from a command-failure dead letter.
#[derive(Debug, Error)]
pub enum PgCommandFailureOutboxDeadLetterRowError {
    #[error("command-failure outbox row error: {0}")]
    Outbox(#[from] PgCommandFailureOutboxRowError),
}
