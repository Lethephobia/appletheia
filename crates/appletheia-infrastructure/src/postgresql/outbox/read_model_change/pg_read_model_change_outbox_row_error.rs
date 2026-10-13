use appletheia_application::outbox::{
    OutboxAttemptCountError, OutboxRelayInstanceError,
    read_model_change::ReadModelChangeOutboxIdError,
};
use appletheia_application::read_model::{ReadModelChangeIdError, ReadModelTypeOwnedError};
use appletheia_domain::EventIdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgReadModelChangeOutboxRowError {
    #[error(transparent)]
    OutboxId(#[from] ReadModelChangeOutboxIdError),

    #[error(transparent)]
    ChangeId(#[from] ReadModelChangeIdError),

    #[error(transparent)]
    SourceEventId(#[from] EventIdError),

    #[error(transparent)]
    ReadModelTypeOwned(#[from] ReadModelTypeOwnedError),

    #[error(transparent)]
    AttemptCount(#[from] OutboxAttemptCountError),

    #[error(transparent)]
    LeaseOwner(#[from] OutboxRelayInstanceError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("outbox row contained inconsistent lease state")]
    InconsistentLeaseState,
}
