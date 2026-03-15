use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur when building a snapshot ID from a raw UUID.
#[derive(Debug, Error)]
pub enum SnapshotIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
