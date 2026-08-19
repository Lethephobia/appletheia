use thiserror::Error;
use uuid::Uuid;

/// Reports an invalid list-chunk identifier.
#[derive(Debug, Error)]
pub enum ReadModelListChunkIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
