use thiserror::Error;

use crate::command::{CommandAttemptCountError, CommandFailureIdError, CommandNameOwnedError};
use crate::messaging::{
    CloudEventAttributeStringError, CloudEventError, CloudEventExtensionNameError,
    CloudEventExtensionsError, CloudEventIdError, CloudEventPartitionKeyError, CloudEventTimeError,
    CloudEventTypeError,
};

#[derive(Debug, Error)]
pub enum CommandFailureCloudEventCodecError {
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
    CloudEventTime(#[from] CloudEventTimeError),

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
