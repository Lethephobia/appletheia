use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SagaInstanceIdError {
    #[error("not a uuidv7: {0}")]
    NotUuidV7(Uuid),
}
