use thiserror::Error;

use super::PgReadModelInvalidationOutboxRowError;

#[derive(Debug, Error)]
pub enum PgReadModelInvalidationOutboxDeadLetterRowError {
    #[error(transparent)]
    OutboxRow(#[from] PgReadModelInvalidationOutboxRowError),
}
