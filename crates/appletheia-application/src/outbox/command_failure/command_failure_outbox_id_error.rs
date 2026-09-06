use thiserror::Error;
use uuid::Uuid;

/// Reports an invalid command-failure outbox identifier.
#[derive(Debug, Error)]
pub enum CommandFailureOutboxIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
