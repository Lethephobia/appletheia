use thiserror::Error;

use appletheia_application::saga::{SagaInstanceIdError, SagaNameOwnedError};
use appletheia_domain::EventIdError;

#[derive(Debug, Error)]
pub(super) enum PgSagaInstanceRowError {
    #[error("saga instance id error: {0}")]
    SagaInstanceId(#[from] SagaInstanceIdError),

    #[error("saga name error: {0}")]
    SagaName(#[from] SagaNameOwnedError),

    #[error("start event id error: {0}")]
    StartEventId(#[from] EventIdError),

    #[error("state deserialization error: {0}")]
    StateDeserialize(#[from] serde_json::Error),
}
