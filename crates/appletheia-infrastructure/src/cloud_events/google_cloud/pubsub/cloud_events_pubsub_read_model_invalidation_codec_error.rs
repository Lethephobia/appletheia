use appletheia_domain::EventIdError;
use thiserror::Error;

use crate::cloud_events::CloudEventSourceError;
use appletheia_application::event::EventSequenceError;
use appletheia_application::projection::ProjectorNameOwnedError;
use appletheia_application::read_model::ReadModelInvalidationIdError;

#[derive(Debug, Error)]
pub enum CloudEventsPubsubReadModelInvalidationCodecError {
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
    EventSequence(#[from] EventSequenceError),

    #[error(transparent)]
    ProjectorNameOwned(#[from] ProjectorNameOwnedError),

    #[error(transparent)]
    ReadModelInvalidationId(#[from] ReadModelInvalidationIdError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("a read-model invalidation must contain at least one partition")]
    EmptyPartitions,
}
