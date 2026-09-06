use thiserror::Error;
use uuid::Uuid;

/// Reports an invalid command execution identifier.
#[derive(Debug, Error)]
pub enum CommandExecutionIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
