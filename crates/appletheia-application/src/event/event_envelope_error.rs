use super::{EventNameOwnedError, EventSequenceError, SerializedEventPayloadError};
use crate::aggregate::AggregateTypeOwnedError;
use crate::messaging::{
    CloudEventAttributeStringError, CloudEventError, CloudEventExtensionNameError,
    CloudEventExtensionsError, CloudEventIdError, CloudEventPartitionKeyError,
    CloudEventSubjectError, CloudEventTimeError, CloudEventTypeError,
};
use appletheia_domain::{AggregateVersionError, EventIdError};
use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventEnvelopeError {
    #[error("missing or invalid CloudEvent metadata: {0}")]
    InvalidMetadata(&'static str),

    #[error(transparent)]
    CloudEventId(#[from] CloudEventIdError),

    #[error(transparent)]
    CloudEventType(#[from] CloudEventTypeError),

    #[error(transparent)]
    CloudEventExtensionName(#[from] CloudEventExtensionNameError),

    #[error(transparent)]
    CloudEventAttributeString(#[from] CloudEventAttributeStringError),

    #[error(transparent)]
    CloudEventExtensions(#[from] CloudEventExtensionsError),

    #[error(transparent)]
    CloudEvent(#[from] CloudEventError),

    #[error(transparent)]
    CloudEventPartitionKey(#[from] CloudEventPartitionKeyError),

    #[error(transparent)]
    CloudEventSubject(#[from] CloudEventSubjectError),

    #[error(transparent)]
    CloudEventTime(#[from] CloudEventTimeError),

    #[error(transparent)]
    EventId(#[from] EventIdError),

    #[error(transparent)]
    AggregateVersion(#[from] AggregateVersionError),

    #[error(transparent)]
    AggregateTypeOwned(#[from] AggregateTypeOwnedError),

    #[error(transparent)]
    EventNameOwned(#[from] EventNameOwnedError),

    #[error(transparent)]
    EventSequence(#[from] EventSequenceError),

    #[error(transparent)]
    SerializedEventPayload(#[from] SerializedEventPayloadError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("event name mismatch: expected {expected}, got {actual}")]
    EventNameMismatch { expected: String, actual: String },

    #[error("aggregate type mismatch: expected {expected}, got {actual}")]
    AggregateTypeMismatch {
        expected: &'static str,
        actual: String,
    },

    #[error("aggregate id error")]
    AggregateId(#[source] Box<dyn Error + Send + Sync>),

    #[error("event payload error")]
    EventPayload(#[source] Box<dyn Error + Send + Sync>),
}
