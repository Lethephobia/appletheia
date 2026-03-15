use thiserror::Error;

use super::PgCommandOutboxRowError;

#[derive(Debug, Error)]
pub enum PgCommandOutboxDeadLetterRowError {
    #[error("command outbox row error: {0}")]
    Outbox(#[from] PgCommandOutboxRowError),
}
