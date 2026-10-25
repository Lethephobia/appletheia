use thiserror::Error;

use super::PgReadModelChangeOutboxRowError;

#[derive(Debug, Error)]
pub enum PgReadModelChangeOutboxDeadLetterRowError {
    #[error("read model change outbox row error: {0}")]
    Outbox(#[from] PgReadModelChangeOutboxRowError),
}
