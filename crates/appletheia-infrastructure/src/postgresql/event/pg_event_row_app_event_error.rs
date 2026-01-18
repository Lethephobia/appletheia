use thiserror::Error;

use appletheia_application::event::{
    AggregateTypeOwnedError, EventPayloadOwnedError, EventSequenceError,
};
use appletheia_domain::aggregate::AggregateVersionError;
use appletheia_domain::event::EventIdError;

#[derive(Debug, Error)]
pub enum PgEventRowAppEventError {
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

    #[error("context deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}

