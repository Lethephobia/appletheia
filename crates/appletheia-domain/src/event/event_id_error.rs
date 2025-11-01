use uuid::Uuid;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
