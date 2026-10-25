use crate::cloud_events::CloudEventSourceError;
use appletheia_application::read_model::ReadModelChangeIdError;
use appletheia_application::read_model::watch::ReadModelWatchNotificationIdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventsPubsubReadModelWatchNotificationCodecError {
    #[error("missing CloudEvents attribute: {0}")]
    MissingAttribute(&'static str),

    #[error("expected CloudEvents specversion 1.0")]
    UnsupportedSpecVersion,

    #[error("expected a JSON content type")]
    InvalidContentType,

    #[error("invalid CloudEvents metadata: {0}")]
    InvalidMetadata(&'static str),

    #[error(transparent)]
    ReadModelChangeId(#[from] ReadModelChangeIdError),

    #[error(transparent)]
    ReadModelWatchNotificationId(#[from] ReadModelWatchNotificationIdError),

    #[error(transparent)]
    CloudEventSource(#[from] CloudEventSourceError),

    #[error(transparent)]
    Timestamp(#[from] chrono::ParseError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
