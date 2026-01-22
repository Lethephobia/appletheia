use thiserror::Error;

use appletheia_application::saga::SagaInstanceIdError;

#[derive(Debug, Error)]
pub(super) enum PgSagaInstanceRowError {
    #[error("saga instance id error: {0}")]
    SagaInstanceId(#[from] SagaInstanceIdError),

    #[error("state deserialization error: {0}")]
    StateDeserialize(#[from] serde_json::Error),

    #[error("invalid persisted saga instance: {message}")]
    InvalidPersistedInstance { message: &'static str },
}
