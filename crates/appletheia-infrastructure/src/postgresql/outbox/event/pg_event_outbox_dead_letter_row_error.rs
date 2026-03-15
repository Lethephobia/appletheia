use thiserror::Error;

use super::PgEventOutboxRowError;

#[derive(Debug, Error)]
pub enum PgEventOutboxDeadLetterRowError {
    #[error("event outbox row error: {0}")]
    Outbox(#[from] PgEventOutboxRowError),
}
