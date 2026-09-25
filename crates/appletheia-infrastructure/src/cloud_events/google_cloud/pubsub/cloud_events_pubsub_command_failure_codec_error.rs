use thiserror::Error;

use crate::cloud_events::CloudEventSourceError;
use appletheia_application::command::{
    CommandAttemptCountError, CommandFailureIdError, CommandNameOwnedError,
};

#[derive(Debug, Error)]
pub enum CloudEventsPubsubCommandFailureCodecError {
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
    CommandNameOwned(#[from] CommandNameOwnedError),

    #[error(transparent)]
    CommandFailureId(#[from] CommandFailureIdError),

    #[error(transparent)]
    CommandAttemptCount(#[from] CommandAttemptCountError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
