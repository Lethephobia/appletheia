use thiserror::Error;
use uuid::Uuid;

/// Reports an invalid command failure identifier.
#[derive(Debug, Error)]
pub enum ReadModelChangeIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
