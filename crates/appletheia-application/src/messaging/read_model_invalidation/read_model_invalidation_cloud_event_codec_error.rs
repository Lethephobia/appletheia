use appletheia_domain::EventIdError;
use thiserror::Error;

use crate::event::EventSequenceError;
use crate::messaging::{
    CloudEventAttributeStringError, CloudEventError, CloudEventExtensionNameError,
    CloudEventExtensionsError, CloudEventIdError, CloudEventPartitionKeyError, CloudEventTimeError,
    CloudEventTypeError,
};
use crate::projection::ProjectorNameOwnedError;
use crate::read_model::ReadModelInvalidationIdError;

#[derive(Debug, Error)]
pub enum ReadModelInvalidationCloudEventCodecError {
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
