use crate::cloud_events::CloudEventSourceError;
use appletheia_application::read_model::{ReadModelChangeIdError, ReadModelTypeOwnedError};
use appletheia_domain::EventIdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventsPubsubReadModelChangeCodecError {
    #[error("missing CloudEvents attribute: {0}")]
    MissingAttribute(&'static str),

    #[error("expected CloudEvents specversion 1.0")]
    UnsupportedSpecVersion,

    #[error("expected a JSON content type")]
    InvalidContentType,

    #[error("CloudEvents type does not match the configured prefix")]
    TypePrefixMismatch,

    #[error("invalid CloudEvents metadata: {0}")]
    InvalidMetadata(&'static str),

    #[error(transparent)]
    ReadModelChangeId(#[from] ReadModelChangeIdError),

    #[error(transparent)]
    SourceEventId(#[from] EventIdError),

    #[error(transparent)]
    ReadModelTypeOwned(#[from] ReadModelTypeOwnedError),

    #[error(transparent)]
    CloudEventSource(#[from] CloudEventSourceError),

    #[error(transparent)]
    Timestamp(#[from] chrono::ParseError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
