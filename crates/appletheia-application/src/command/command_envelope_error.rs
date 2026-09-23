use super::CommandNameOwnedError;
use crate::messaging::{
    CloudEventAttributeStringError, CloudEventError, CloudEventExtensionNameError,
    CloudEventExtensionsError, CloudEventIdError, CloudEventPartitionKeyError, CloudEventTypeError,
};
use thiserror::Error;

use super::SerializedCommandError;

#[derive(Debug, Error)]
pub enum CommandEnvelopeError {
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
    CommandNameOwned(#[from] CommandNameOwnedError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error("command name mismatch: expected {expected}, got {actual}")]
    CommandNameMismatch { expected: String, actual: String },

    #[error("json deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid serialized command: {0}")]
    SerializedCommand(#[from] SerializedCommandError),
}
