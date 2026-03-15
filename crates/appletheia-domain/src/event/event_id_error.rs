use uuid::Uuid;

use thiserror::Error;

/// Errors that can occur when building an event ID from a raw UUID.
#[derive(Debug, Error)]
pub enum EventIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
