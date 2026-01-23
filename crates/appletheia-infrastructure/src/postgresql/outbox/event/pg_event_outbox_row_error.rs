use thiserror::Error;

use appletheia_application::event::{EventPayloadOwnedError, EventSequenceError};
use appletheia_application::outbox::{
    OrderingKeyError, OutboxAttemptCountError, OutboxRelayInstanceError, event::EventOutboxIdError,
};
use appletheia_domain::aggregate::AggregateVersionError;
use appletheia_domain::event::EventIdError;

#[derive(Debug, Error)]
pub enum PgEventOutboxRowError {
    #[error("outbox id error: {0}")]
    OutboxId(#[from] EventOutboxIdError),

    #[error("event sequence error: {0}")]
    EventSequence(#[from] EventSequenceError),

    #[error("event id error: {0}")]
    EventId(#[from] EventIdError),

    #[error("aggregate type error: {0}")]
    AggregateType(String),

    #[error("aggregate version error: {0}")]
    AggregateVersion(#[from] AggregateVersionError),

    #[error("payload error: {0}")]
    Payload(#[from] EventPayloadOwnedError),

    #[error("attempt count error: {0}")]
    AttemptCount(#[from] OutboxAttemptCountError),

    #[error("lease owner error: {0}")]
    LeaseOwner(#[from] OutboxRelayInstanceError),

    #[error("ordering key error: {0}")]
    OrderingKey(#[from] OrderingKeyError),

    #[error("context deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("outbox row contained inconsistent lease state")]
    InconsistentLeaseState,
}
