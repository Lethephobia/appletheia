use thiserror::Error;
use uuid::Uuid;

/// Reports an invalid fragment-change identifier.
#[derive(Debug, Error)]
pub enum ReadModelFragmentChangeIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
