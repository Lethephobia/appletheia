use thiserror::Error;

use appletheia_application::event::{
    AggregateTypeOwnedError, EventPayloadOwnedError, EventSequenceError,
};
use appletheia_application::outbox::{
    OutboxAttemptCountError, OutboxIdError, OutboxRelayInstanceError,
};
use appletheia_domain::aggregate::AggregateVersionError;
use appletheia_domain::event::EventIdError;

#[derive(Debug, Error)]
pub enum PgOutboxRowError {
    #[error("outbox id error: {0}")]
    OutboxId(#[from] OutboxIdError),

    #[error("event sequence error: {0}")]
    EventSequence(#[from] EventSequenceError),

    #[error("event id error: {0}")]
    EventId(#[from] EventIdError),

    #[error("aggregate type error: {0}")]
    AggregateType(#[from] AggregateTypeOwnedError),

    #[error("aggregate version error: {0}")]
    AggregateVersion(#[from] AggregateVersionError),

    #[error("payload error: {0}")]
    Payload(#[from] EventPayloadOwnedError),

    #[error("request context error: {0}")]
    RequestContext(#[from] serde_json::Error),

    #[error("attempt count error: {0}")]
    AttemptCount(#[from] OutboxAttemptCountError),

    #[error("lease owner error: {0}")]
    LeaseOwner(#[from] OutboxRelayInstanceError),
}
