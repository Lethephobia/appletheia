use uuid::Uuid;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelInvalidationOutboxIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
