use thiserror::Error;
use uuid::Uuid;

/// Reports an invalid read-model invalidation identifier.
#[derive(Debug, Error)]
pub enum ReadModelInvalidationIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
