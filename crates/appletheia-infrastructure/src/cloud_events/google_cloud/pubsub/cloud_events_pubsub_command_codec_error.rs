use thiserror::Error;

use crate::cloud_events::CloudEventSourceError;
use appletheia_application::command::{CommandNameOwnedError, SerializedCommandError};

#[derive(Debug, Error)]
pub enum CloudEventsPubsubCommandCodecError {
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

    #[error("missing or invalid CloudEvent metadata: {0}")]
    InvalidMetadata(&'static str),

    #[error(transparent)]
    CommandNameOwned(#[from] CommandNameOwnedError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error("json deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid serialized command: {0}")]
    SerializedCommand(#[from] SerializedCommandError),
}
