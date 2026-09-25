use thiserror::Error;

use appletheia_application::saga::SagaInstanceIdError;
use appletheia_domain::EventIdError;

#[derive(Debug, Error)]
pub(super) enum PgSagaInstanceRowError {
    #[error("saga instance id error: {0}")]
    SagaInstanceId(#[from] SagaInstanceIdError),

    #[error("start event id error: {0}")]
    StartEventId(#[from] EventIdError),

    #[error("state deserialization error: {0}")]
    StateDeserialize(#[from] serde_json::Error),
}
