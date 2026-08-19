use thiserror::Error;

use appletheia_application::event::EventSequenceError;
use appletheia_application::outbox::{OutboxAttemptCountError, OutboxRelayInstanceError};
use appletheia_application::projection::ProjectorNameOwnedError;
use appletheia_application::read_model::{
    ReadModelInvalidationEnvelopeError, ReadModelInvalidationIdError,
};
use appletheia_domain::EventIdError;

/// Reports invalid state or values loaded from the invalidation outbox.
#[derive(Debug, Error)]
pub enum PgReadModelInvalidationOutboxRowError {
    #[error("invalidation id error: {0}")]
    InvalidationId(#[from] ReadModelInvalidationIdError),
    #[error("source projector name error: {0}")]
    SourceProjectorName(#[from] ProjectorNameOwnedError),
    #[error("source event sequence error: {0}")]
    SourceEventSequence(#[from] EventSequenceError),
    #[error("source event id error: {0}")]
    SourceEventId(#[from] EventIdError),
    #[error("attempt count error: {0}")]
    AttemptCount(#[from] OutboxAttemptCountError),
    #[error("lease owner error: {0}")]
    LeaseOwner(#[from] OutboxRelayInstanceError),
    #[error("json mapping error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalidation envelope error: {0}")]
    Envelope(#[from] ReadModelInvalidationEnvelopeError),
    #[error("outbox row contained inconsistent lease state")]
    InconsistentLeaseState,
}
