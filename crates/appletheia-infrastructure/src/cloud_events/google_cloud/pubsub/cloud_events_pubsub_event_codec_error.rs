use appletheia_domain::{AggregateVersionError, EventIdError};
use thiserror::Error;

use crate::cloud_events::CloudEventSourceError;
use appletheia_application::aggregate::AggregateTypeOwnedError;
use appletheia_application::event::{
    EventNameOwnedError, EventSequenceError, SerializedEventPayloadError,
};

#[derive(Debug, Error)]
pub enum CloudEventsPubsubEventCodecError {
    #[error("missing CloudEvents attribute: {0}")]
    MissingAttribute(&'static str),

    #[error("expected CloudEvents specversion 1.0")]
    UnsupportedSpecVersion,

    #[error("expected a JSON content type")]
    InvalidContentType,

    #[error("CloudEvents type does not match the configured prefix")]
    TypePrefixMismatch,

    #[error(transparent)]
    CloudEventSource(#[from] CloudEventSourceError),

    #[error(transparent)]
    Timestamp(#[from] chrono::ParseError),

    #[error("missing or invalid CloudEvent metadata: {0}")]
    InvalidMetadata(&'static str),

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
}
