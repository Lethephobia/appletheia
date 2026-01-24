use std::error::Error;

use thiserror::Error;

use appletheia_application::event::{EventSequenceError, SerializedEventPayloadError};
use appletheia_domain::{AggregateVersionError, EventIdError};

#[derive(Debug, Error)]
pub enum PgEventRowError {
    #[error("event sequence error: {0}")]
    EventSequence(#[from] EventSequenceError),

    #[error("event id error: {0}")]
    EventId(#[from] EventIdError),

    #[error("aggregate type error: {0}")]
    AggregateType(String),

    #[error("aggregate id error: {0}")]
    AggregateId(#[source] Box<dyn Error + Send + Sync>),

    #[error("aggregate version error: {0}")]
    AggregateVersion(#[from] AggregateVersionError),

    #[error("event payload error: {0}")]
    EventPayload(#[source] Box<dyn Error + Send + Sync>),

    #[error("payload error: {0}")]
    Payload(#[from] SerializedEventPayloadError),

    #[error("context deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}
